use core::cell::OnceCell;
use itertools::Itertools;
use phantom_zone_evaluator::boolean::{fhew::param::I_4P_60, fhew::prelude::*, FheBool};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use std::env;

use phantom_benchs::*;

// type Evaluator = FhewBoolEvaluator<NoisyNativeRing, NonNativePowerOfTwoRing>;
//
// fn encrypt_bool<'a>(
//     evaluator: &'a Evaluator,
//     sk: &LweSecretKeyOwned<i32>,
//     m: bool,
//     rng: &mut LweRng<impl RngCore, impl RngCore>,
// ) -> FheBool<&'a Evaluator> {
//     let ct = FhewBoolCiphertext::sk_encrypt(evaluator.param(), evaluator.ring(), sk, m, rng);
//     FheBool::new(evaluator, ct)
// }
//
// fn decrypt_bool(
//     evaluator: &Evaluator,
//     sk: &LweSecretKeyOwned<i32>,
//     ct: FheBool<Evaluator>,
// ) -> bool {
//     ct.ct().decrypt(evaluator.ring(), sk)
// }

fn u64_to_binary<const N: usize>(v: u64) -> Vec<bool> {
    assert!((v as u128) < 2u128.pow(N as u32));
    let mut result = vec![false; N];
    for i in 0..N {
        if (v >> i) & 1 == 1 {
            result[i] = true;
        }
    }
    result
}

#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
struct Client<R: RingOps, M: ModulusOps> {
    param: FhewBoolMpiParam,
    crs: FhewBoolMpiCrs<StdRng>,
    share_idx: usize,
    sk_seed: <StdRng as SeedableRng>::Seed,
    pk: RlwePublicKeyOwned<R::Elem>,
    #[serde(skip)]
    ring: OnceCell<R>,
    #[serde(skip)]
    mod_ks: OnceCell<M>,
}

impl<R: RingOps, M: ModulusOps> Client<R, M> {
    fn new(param: FhewBoolMpiParam, crs: FhewBoolMpiCrs<StdRng>, share_idx: usize) -> Self {
        let mut sk_seed = <StdRng as SeedableRng>::Seed::default();
        StdRng::from_entropy().fill_bytes(sk_seed.as_mut());
        Self {
            param,
            crs,
            share_idx,
            sk_seed,
            pk: RlwePublicKey::allocate(param.ring_size),
            ring: Default::default(),
            mod_ks: Default::default(),
        }
    }

    fn ring(&self) -> &R {
        self.ring
            .get_or_init(|| RingOps::new(self.param.modulus, self.param.ring_size))
    }

    fn mod_ks(&self) -> &M {
        self.mod_ks.get_or_init(|| M::new(self.param.lwe_modulus))
    }

    fn sk(&self) -> RlweSecretKeyOwned<i64> {
        RlweSecretKey::sample(
            self.param.ring_size,
            self.param.sk_distribution,
            &mut StdRng::from_hierarchical_seed(self.sk_seed, &[0]),
        )
    }

    fn sk_ks(&self) -> LweSecretKeyOwned<i64> {
        LweSecretKey::sample(
            self.param.lwe_dimension,
            self.param.lwe_sk_distribution,
            &mut StdRng::from_hierarchical_seed(self.sk_seed, &[1]),
        )
    }

    fn pk_share_gen(&self) -> SeededRlwePublicKeyOwned<R::Elem> {
        let mut pk = SeededRlwePublicKey::allocate(self.param.ring_size);
        pk_share_gen(
            self.ring(),
            &mut pk,
            &self.param,
            &self.crs,
            &self.sk(),
            &mut StdRng::from_entropy(),
        );
        pk
    }

    fn receive_pk(&mut self, pk: &RlwePublicKeyOwned<R::Elem>) {
        self.pk = pk.cloned();
    }

    fn bs_key_share_gen(&self) -> FhewBoolMpiKeyShareOwned<R::Elem, M::Elem> {
        let mut bs_key_share = FhewBoolMpiKeyShareOwned::allocate(self.param, self.share_idx);
        bs_key_share_gen(
            self.ring(),
            self.mod_ks(),
            &mut bs_key_share,
            &self.crs,
            &self.sk(),
            &self.pk,
            &self.sk_ks(),
            &mut StdRng::from_entropy(),
        );
        bs_key_share
    }

    // fn decrypt_share(
    //     &self,
    //     ct: [FhewBoolCiphertextOwned<R::Elem>; 8],
    // ) -> [LweDecryptionShare<R::Elem>; 8] {
    //     ct.map(|ct| {
    //         ct.decrypt_share(
    //             &self.param,
    //             self.ring(),
    //             self.sk().as_view(),
    //             &mut StdLweRng::from_entropy(),
    //         )
    //     })
    // }
}

#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
struct Server<R: RingOps, M: ModulusOps> {
    param: FhewBoolMpiParam,
    crs: FhewBoolMpiCrs<StdRng>,
    pk: RlwePublicKeyOwned<R::Elem>,
    #[serde(rename = "bs_key")]
    evaluator: FhewBoolEvaluator<R, M>,
}

impl<R: RingOps, M: ModulusOps> Server<R, M> {
    fn new(param: FhewBoolMpiParam) -> Self {
        Self {
            param,
            crs: FhewBoolMpiCrs::sample(StdRng::from_entropy()),
            pk: RlwePublicKey::allocate(param.ring_size),
            evaluator: FhewBoolEvaluator::new(FhewBoolKeyOwned::allocate(*param)),
        }
    }

    fn ring(&self) -> &R {
        self.evaluator.ring()
    }

    fn mod_ks(&self) -> &M {
        self.evaluator.mod_ks()
    }

    fn aggregate_pk_shares(&mut self, pk_shares: &[SeededRlwePublicKeyOwned<R::Elem>]) {
        aggregate_pk_shares(self.evaluator.ring(), &mut self.pk, &self.crs, pk_shares);
    }

    fn aggregate_bs_key_shares<R2: RingOps<Elem = R::Elem>>(
        &mut self,
        bs_key_shares: &[FhewBoolMpiKeyShareOwned<R::Elem, M::Elem>],
    ) {
        let bs_key = {
            let ring = <R2 as RingOps>::new(self.param.modulus, self.param.ring_size);
            let mut bs_key = FhewBoolKeyOwned::allocate(*self.param);
            aggregate_bs_key_shares(&ring, self.mod_ks(), &mut bs_key, &self.crs, bs_key_shares);
            bs_key
        };
        let bs_key_prep = {
            let mut bs_key_prep =
                FhewBoolKeyOwned::allocate_eval(*self.param, self.ring().eval_size());
            prepare_bs_key(self.ring(), &mut bs_key_prep, &bs_key);
            bs_key_prep
        };
        self.evaluator = FhewBoolEvaluator::new(bs_key_prep);
    }

    // fn pk_encrypt(&self, m: u8) -> [FhewBoolCiphertextOwned<R::Elem>; 8] {
    //     pk_encrypt(&self.param, self.ring(), &self.pk, m)
    // }
}

// fn pk_encrypt<R: RingOps>(
//     param: &FhewBoolParam,
//     ring: &R,
//     pk: &RlwePublicKeyOwned<R::Elem>,
//     m: u8,
// ) -> [FhewBoolCiphertextOwned<R::Elem>; 8] {
//     FhewBoolBatchedCiphertext::pk_encrypt(
//         param,
//         ring,
//         pk,
//         (0..8).map(|idx| (m >> idx) & 1 == 1),
//         &mut StdLweRng::from_entropy(),
//     )
//     .extract_all(ring)
//     .try_into()
//     .unwrap()
// }

// fn aggregate_decryption_shares<R: RingOps>(
//     ring: &R,
//     ct: [FhewBoolCiphertextOwned<R::Elem>; 8],
//     dec_shares: &[[LweDecryptionShare<R::Elem>; 8]],
// ) -> u8 {
//     (0..8)
//         .map(|idx| {
//             let dec_shares = dec_shares.iter().map(|dec_shares| &dec_shares[idx]);
//             ct[idx].aggregate_decryption_shares(ring, dec_shares)
//         })
//         .rev()
//         .fold(0, |m, b| (m << 1) | b as u8)
// }

fn serialize_pk_share<R: RingOps>(
    ring: &R,
    pk_share: &SeededRlwePublicKeyOwned<R::Elem>,
) -> Vec<u8> {
    bincode::serialize(&pk_share.compact(ring)).unwrap()
}

fn deserialize_pk_share<R: RingOps>(ring: &R, bytes: &[u8]) -> SeededRlwePublicKeyOwned<R::Elem> {
    let pk_share_compact: SeededRlwePublicKey<Compact> = bincode::deserialize(bytes).unwrap();
    pk_share_compact.uncompact(ring)
}

fn serialize_pk<R: RingOps>(ring: &R, pk: &RlwePublicKeyOwned<R::Elem>) -> Vec<u8> {
    bincode::serialize(&pk.compact(ring)).unwrap()
}

fn deserialize_pk<R: RingOps>(ring: &R, bytes: &[u8]) -> RlwePublicKeyOwned<R::Elem> {
    let pk_compact: RlwePublicKey<Compact> = bincode::deserialize(bytes).unwrap();
    pk_compact.uncompact(ring)
}

fn serialize_bs_key_share<R: RingOps, M: ModulusOps>(
    ring: &R,
    mod_ks: &M,
    bs_key_share: &FhewBoolMpiKeyShareOwned<R::Elem, M::Elem>,
) -> Vec<u8> {
    bincode::serialize(&bs_key_share.compact(ring, mod_ks)).unwrap()
}

fn deserialize_bs_key_share<R: RingOps, M: ModulusOps>(
    ring: &R,
    mod_ks: &M,
    bytes: &[u8],
) -> FhewBoolMpiKeyShareOwned<R::Elem, M::Elem> {
    let bs_key_share_compact: FhewBoolMpiKeyShareCompact = bincode::deserialize(bytes).unwrap();
    bs_key_share_compact.uncompact(ring, mod_ks)
}

// fn serialize_cts<R: RingOps>(ring: &R, cts: [FhewBoolCiphertextOwned<R::Elem>; 8]) -> Vec<u8> {
//     bincode::serialize(&cts.map(|ct| ct.compact(ring))).unwrap()
// }
//
// fn deserialize_cts<R: RingOps>(ring: &R, bytes: &[u8]) -> [FhewBoolCiphertextOwned<R::Elem>; 8] {
//     let cts: [FhewBoolCiphertext<Compact>; 8] = bincode::deserialize(bytes).unwrap();
//     cts.map(|ct| ct.uncompact(ring))
// }

enum Bench {
    FZApplyMove,
    FZGetCell,
    FZGetCrossCells,
    FZGetFiveCells,
    FZGetHorizontalCells,
    FZGetVerticalCells,
}

const NUM_OBSTACLES: usize = 100;
const NUM_PLAYERS: usize = 4;
const NUM_ITEMS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        bits.extend_from_slice(&u64_to_binary::<8>(self.x as u64));
        bits.extend_from_slice(&u64_to_binary::<8>(self.y as u64));
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Coords5 {
    pub values: [Coord; 5],
}

impl Coords5 {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for value in &self.values {
            bits.extend_from_slice(&value.to_bits());
        }
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PlayerData {
    pub loc: Coord,
    pub hp: u8,
    pub atk: u8,
}

impl PlayerData {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        bits.extend_from_slice(&self.loc.to_bits());
        bits.extend_from_slice(&u64_to_binary::<8>(self.hp as u64));
        bits.extend_from_slice(&u64_to_binary::<8>(self.atk as u64));
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PlayerWithId {
    pub id: u8,
    pub data: PlayerData,
}

impl PlayerWithId {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        bits.extend_from_slice(&u64_to_binary::<8>(self.id as u64));
        bits.extend_from_slice(&self.data.to_bits());
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PlayersWithId {
    pub values: [PlayerWithId; NUM_PLAYERS],
}

impl PlayersWithId {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for value in &self.values {
            bits.extend_from_slice(&value.to_bits());
        }
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Up = 0b00,
    Down = 0b01,
    Left = 0b10,
    Right = 0b11,
}

impl Direction {
    pub fn to_bits(&self) -> Vec<bool> {
        u64_to_binary::<2>(*self as u64)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Obstacles100 {
    pub values: [Coord; NUM_OBSTACLES],
}

impl Default for Obstacles100 {
    fn default() -> Self {
        Self {
            values: [(); NUM_OBSTACLES].map(|_| Coord::default()),
        }
    }
}

impl Obstacles100 {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for value in &self.values {
            bits.extend_from_slice(&value.to_bits());
        }
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ItemData {
    pub loc: Coord,
    pub hp: u8,
    pub atk: u8,
    pub is_consumed: bool,
}

impl ItemData {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        bits.extend_from_slice(&self.loc.to_bits());
        bits.extend_from_slice(&u64_to_binary::<8>(self.hp as u64));
        bits.extend_from_slice(&u64_to_binary::<8>(self.atk as u64));
        bits.extend_from_slice(&u64_to_binary::<1>(self.is_consumed as u64));
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Items {
    pub values: [ItemData; NUM_ITEMS],
}

impl Items {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for value in &self.values {
            bits.extend_from_slice(&value.to_bits());
        }
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ItemWithId {
    pub id: u8,
    pub data: ItemData,
}

impl ItemWithId {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        bits.extend_from_slice(&u64_to_binary::<8>(self.id as u64));
        bits.extend_from_slice(&self.data.to_bits());
        bits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ItemsWithId {
    pub values: [ItemWithId; NUM_ITEMS],
}

impl ItemsWithId {
    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for value in &self.values {
            bits.extend_from_slice(&value.to_bits());
        }
        bits
    }
}

fn main() {
    let bench_str = env::var("BENCH").unwrap();
    let bench = match bench_str.as_str() {
        "FZApplyMove" => Bench::FZApplyMove,
        "FZGetCell" => Bench::FZGetCell,
        "FZGetCrossCells" => Bench::FZGetCrossCells,
        "FZGetFiveCells" => Bench::FZGetFiveCells,
        "FZGetHorizontalCells" => Bench::FZGetHorizontalCells,
        "FZGetVerticalCells" => Bench::FZGetVerticalCells,
        _ => {
            println!("Invalid bench {}", bench_str);
            std::process::exit(1);
        }
    };

    let mut server = Server::<NoisyNativeRing, NonNativePowerOfTwo>::new(I_4P_60);
    let mut clients = (0..server.param.total_shares)
        .map(|share_idx| {
            Client::<NativeRing, NonNativePowerOfTwo>::new(server.param, server.crs, share_idx)
        })
        .collect_vec();

    // Round 1

    // Clients generate public key shares
    let pk_shares = clients
        .iter()
        .map(|client| serialize_pk_share(client.ring(), &client.pk_share_gen()))
        .collect_vec();

    // Server aggregates public key shares
    server.aggregate_pk_shares(
        &pk_shares
            .into_iter()
            .map(|bytes| deserialize_pk_share(server.ring(), &bytes))
            .collect_vec(),
    );
    let pk = serialize_pk(server.ring(), &server.pk);

    // Round 2

    // Clients generate bootstrapping key shares
    let bs_key_shares = clients
        .iter_mut()
        .map(|client| {
            client.receive_pk(&deserialize_pk(client.ring(), &pk));
            serialize_bs_key_share(client.ring(), client.mod_ks(), &client.bs_key_share_gen())
        })
        .collect_vec();

    // Server aggregates bootstrapping key shares
    server.aggregate_bs_key_shares::<NativeRing>(
        &bs_key_shares
            .into_iter()
            .map(|bytes| deserialize_bs_key_share(server.ring(), server.mod_ks(), &bytes))
            .collect_vec(),
    );

    // Server performs FHE evaluation
    let inputs = match bench {
        Bench::FZApplyMove => {
            // (direction, items, obstacles, player_data)
            // (Direction, Items, Obstacles100, PlayerData)
            vec![
                Direction::default().to_bits(),
                Items::default().to_bits(),
                Obstacles100::default().to_bits(),
                PlayerData::default().to_bits(),
            ]
        }
        Bench::FZGetCell => {
            // (items, player_coord, players, query_coord)
            // (ItemsWithId, Coord, PlayersWithId, Coord)
            vec![
                ItemsWithId::default().to_bits(),
                Coord::default().to_bits(),
                PlayersWithId::default().to_bits(),
                Coord::default().to_bits(),
            ]
        }
        Bench::FZGetCrossCells => {
            // (items, player_coord, players)
            // (ItemsWithId, Coord, PlayersWithId)
            vec![
                ItemsWithId::default().to_bits(),
                Coord::default().to_bits(),
                PlayersWithId::default().to_bits(),
            ]
        }
        Bench::FZGetFiveCells => {
            // (items, player_coord, players, query_coords)
            // (Coord, Coords5, ItemsWithId, PlayersWithId)
            vec![
                ItemsWithId::default().to_bits(),
                Coord::default().to_bits(),
                PlayersWithId::default().to_bits(),
                Coords5::default().to_bits(),
            ]
        }
        Bench::FZGetHorizontalCells => {
            // (items, player_coord, players, query_coord)
            // (ItemsWithId, Coord, PlayersWithId, Coord)
            vec![
                ItemsWithId::default().to_bits(),
                Coord::default().to_bits(),
                PlayersWithId::default().to_bits(),
                Coord::default().to_bits(),
            ]
        }
        Bench::FZGetVerticalCells => {
            // (items, player_coord, players, query_coord)
            // (ItemsWithId, Coord, PlayersWithId, Coord)
            vec![
                ItemsWithId::default().to_bits(),
                Coord::default().to_bits(),
                PlayersWithId::default().to_bits(),
                Coord::default().to_bits(),
            ]
        }
    };

    let mut rng = StdLweRng::from_entropy();

    let now = std::time::Instant::now();
    let inputs_enc = inputs
        .iter()
        .map(|xs| {
            xs.iter()
                .map(|x| {
                    let ct = FhewBoolCiphertext::pk_encrypt(
                        server.evaluator.param(),
                        server.evaluator.ring(),
                        &server.pk,
                        *x,
                        &mut rng,
                    );
                    FheBool::new(&server.evaluator, ct)
                })
                .collect_vec()
        })
        .collect_vec();
    println!("Client cyphertext encryption time: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    // https://hackmd.io/TjTYc-86QxGuxixpRbhTdA?view
    let outputs_enc = match bench {
        Bench::FZApplyMove => vec![frogzone_apply_move_rs_fhe_lib::apply_move(
            &inputs_enc[0],
            &inputs_enc[1],
            &inputs_enc[2],
            &inputs_enc[3],
        )],
        Bench::FZGetCell => vec![frogzone_get_cell_rs_fhe_lib::get_cell(
            &inputs_enc[0],
            &inputs_enc[1],
            &inputs_enc[2],
            &inputs_enc[3],
        )],
        Bench::FZGetCrossCells => vec![frogzone_get_cross_cells_rs_fhe_lib::get_cross_cells(
            &inputs_enc[0],
            &inputs_enc[1],
            &inputs_enc[2],
        )],
        Bench::FZGetFiveCells => vec![frogzone_get_five_cells_rs_fhe_lib::get_five_cells(
            &inputs_enc[0],
            &inputs_enc[1],
            &inputs_enc[2],
            &inputs_enc[3],
        )],
        Bench::FZGetHorizontalCells => {
            vec![
                frogzone_get_horizontal_cells_rs_fhe_lib::get_horizontal_cells(
                    &inputs_enc[0],
                    &inputs_enc[1],
                    &inputs_enc[2],
                    &inputs_enc[3],
                ),
            ]
        }
        Bench::FZGetVerticalCells => {
            vec![frogzone_get_vertical_cells_rs_fhe_lib::get_vertical_cells(
                &inputs_enc[0],
                &inputs_enc[1],
                &inputs_enc[2],
                &inputs_enc[3],
            )]
        }
    };
    println!("FHE circuit evaluation time: {:?}", now.elapsed());

    // let output_enc_bin = {
    //     let [a, b, c, d, e] =
    //         &input.map(|m| FheU8::from_cts(&server.evaluator, server.pk_encrypt(m)));
    //     serialize_cts(server.ring(), function(a, b, c, d, e).into_cts())
    // };

    // // Clients generate decryption share of evaluation output
    // let ct_g_dec_shares = clients
    //     .iter()
    //     .map(|client| client.decrypt_share(deserialize_cts(client.ring(), &output_enc_bin)))
    //     .collect_vec();

    // // Aggregate decryption shares
    // assert_eq!(output, {
    //     let output_enc = deserialize_cts(clients[0].ring(), &output_enc_bin);
    //     aggregate_decryption_shares(clients[0].ring(), output_enc, &ct_g_dec_shares)
    // });

    /*
    let sk = LweSecretKey::sample(PARAM.ring_size, PARAM.sk_distribution, &mut rng);
    let evaluator = Evaluator::sample(PARAM, &sk, &mut rng);

    // Function with bools by gate-level operations
    let m = u64_to_binary::<32>(1234);
    // let g = {
    //     let input: Vec<_> = m.iter().map(|m| *m.into()).collect();
    //     gate_level_function::<MockBoolEvaluator>(&input)
    // };
    let now = std::time::Instant::now();
    let input: Vec<_> = m
        .iter()
        .map(|m| encrypt_bool(&evaluator, &sk, *m, &mut rng))
        .collect();
    println!("Client cyphertext encryption time: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    let ct_g = demo_rs_fhe_lib::fibonacci_number(&input);
    println!("FHE circuit evaluation time: {:?}", now.elapsed());

    // for (g, ct_g) in g.iter().zip(ct_g.iter()) {
    //     assert_eq!(g, decrypt_bool(&evaluator, &sk, ct_g));
    // }
    */
}
