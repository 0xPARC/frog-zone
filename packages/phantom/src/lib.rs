use core::{fmt::Debug, ops::Deref};
use phantom_zone_evaluator::boolean::fhew::{param::I_4P_60, prelude::*};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "dev")]
pub mod dev;

/// Parameter shared between server and users.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhantomParam {
    param: FhewBoolMpiParam,
    ring_packing_modulus: Option<Modulus>,
    ring_packing_auto_decomposition_param: DecompositionParam,
    crs: PhantomCrs,
}

#[wasm_bindgen]
impl PhantomParam {
    #[wasm_bindgen]
    pub fn i_4p_60() -> Self {
        Self {
            param: I_4P_60,
            ring_packing_modulus: Some(Modulus::Prime(2305843009213554689)),
            ring_packing_auto_decomposition_param: DecompositionParam {
                log_base: 20,
                level: 1,
            },
            crs: PhantomCrs::new(*b"0xPARC0xPARC0xPARC0xPARC0xPARC0x"),
        }
    }
}

impl Deref for PhantomParam {
    type Target = FhewBoolMpiParam;

    fn deref(&self) -> &Self::Target {
        &self.param
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhantomCrs(<StdRng as SeedableRng>::Seed);

impl PhantomCrs {
    pub fn new(seed: <StdRng as SeedableRng>::Seed) -> Self {
        Self(seed)
    }

    pub fn from_entropy() -> Self {
        Self::new(thread_rng().gen())
    }

    fn fhew(&self) -> FhewBoolMpiCrs<StdRng> {
        FhewBoolMpiCrs::new(StdRng::from_hierarchical_seed(self.0, &[0]).gen())
    }

    fn ring_packing(&self) -> RingPackingCrs<StdRng> {
        RingPackingCrs::new(StdRng::from_hierarchical_seed(self.0, &[1]).gen())
    }
}

/// Round 1 key share during key generation, containing public key share.
pub type PhantomRound1Key = PhantomPkShare;

/// Round 2 key share during key generation, containing ring-packing key share
/// and bootstrapping key share.
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhantomRound2Key {
    rp_key_share: PhantomRpKeyShare,
    bs_key_share: PhantomBsKeyShare,
}

/// [`PhantomUser`] proivdes necessary functionality to do deterministic key
/// generation given a seed, encryption, decryption share generation, and
/// decryption shares aggregation.
#[wasm_bindgen]
pub struct PhantomUser {
    ops: PhantomOps,
    user_id: usize,
    seed: <StdRng as SeedableRng>::Seed,
}

#[wasm_bindgen]
impl PhantomUser {
    /// Returns a new [`PhantomUser`].
    #[wasm_bindgen]
    pub fn new(param: PhantomParam, user_id: usize, seed: Vec<u8>) -> Self {
        Self {
            ops: PhantomOps::new(param),
            user_id,
            seed: seed.try_into().unwrap(),
        }
    }

    /// Returns user id.
    #[wasm_bindgen]
    pub fn user_id(&self) -> usize {
        self.user_id
    }

    /// Generates round 1 key.
    #[wasm_bindgen]
    pub fn round_1_key_gen(&self) -> PhantomRound1Key {
        self.ops
            .pk_share_gen(&self.sk(), self.deterministic_rng(&[1, 0]))
    }

    /// Returns if aggregated public key is set or not.
    #[wasm_bindgen]
    pub fn has_pk(&self) -> bool {
        self.ops.pk.is_some()
    }

    /// Sets aggregated public key retrieved from server.
    #[wasm_bindgen]
    pub fn set_pk(&mut self, pk: PhantomPk) {
        self.ops.pk = Some(pk)
    }

    /// Generates round 2 key.
    ///
    /// # Panics
    ///
    /// Panics if [`PhantomUser::set_pk`] is not called yet.
    #[wasm_bindgen]
    pub fn round_2_key_gen(&self) -> PhantomRound2Key {
        PhantomRound2Key {
            bs_key_share: self.ops.bs_key_share_gen(
                self.user_id,
                &self.sk(),
                &self.sk_ks(),
                self.deterministic_rng(&[1, 2]),
            ),
            rp_key_share: self
                .ops
                .rp_key_share_gen(&self.sk(), self.deterministic_rng(&[1, 1])),
        }
    }

    /// Encrypts bits in batch.
    ///
    /// # Panics
    ///
    /// Panics if [`PhantomUser::set_pk`] is not called yet, or any input `bits`
    /// is not `0` or `1`.
    #[wasm_bindgen]
    pub fn batched_pk_encrypt(&self, bits: Vec<u8>) -> PhantomBatchedCt {
        self.ops.batched_pk_encrypt(bits.into_iter().map(|bit| {
            assert!(bit == 0 || bit == 1);
            bit == 1
        }))
    }

    /// Generates decryption share.
    #[wasm_bindgen]
    pub fn decrypt_share(&self, ct_packed: &PhantomPackedCt) -> PhantomPackedCtDecShare {
        self.ops.decrypt_share(&self.sk(), ct_packed)
    }

    /// Aggregates decryption share and returns decrypted bits.
    #[wasm_bindgen]
    pub fn aggregate_dec_shares(
        &self,
        ct_packed: &PhantomPackedCt,
        dec_shares: Vec<PhantomPackedCtDecShare>,
    ) -> Vec<u8> {
        self.ops
            .aggregate_dec_shares(ct_packed, &dec_shares)
            .into_iter()
            .map(|bit| bit as u8)
            .collect()
    }

    fn sk(&self) -> PhantomSk {
        self.ops
            .sk_gen(StdRng::from_hierarchical_seed(self.seed, &[0, 0]))
    }

    fn sk_ks(&self) -> PhantomSkKs {
        self.ops
            .sk_ks_gen(StdRng::from_hierarchical_seed(self.seed, &[0, 1]))
    }

    fn deterministic_rng(&self, path: &[usize]) -> StdRng {
        StdRng::from_hierarchical_seed(self.seed, path)
    }
}

/// [`PhantomEvaluator`] provides necessary functionality to do key aggregation,
/// compuation on encrypted bits, packing of encrypted bits.
#[derive(Clone, Debug)]
pub struct PhantomEvaluator {
    ops: PhantomOps,
    rp_key: Option<PhantomRpKey>,
    rp_key_prep: Option<PhantomRpKeyPrep>,
    bs_key: Option<PhantomBsKey>,
    evaluator: Option<Arc<FhewBoolEvaluator<EvaluationRing, KeySwitchMod>>>,
}

impl PhantomEvaluator {
    /// Returns a new [`PhantomEvaluator`].
    pub fn new(param: PhantomParam) -> Self {
        Self {
            ops: PhantomOps::new(param),
            rp_key: None,
            rp_key_prep: None,
            bs_key: None,
            evaluator: None,
        }
    }

    /// Returns [`Option`] of aggregated public key.
    pub fn pk(&self) -> Option<&PhantomPk> {
        self.ops.pk.as_ref()
    }

    /// Returns [`Option`] of aggregated ring-packing key.
    pub fn rp_key(&self) -> Option<&PhantomRpKey> {
        self.rp_key.as_ref()
    }

    /// Returns [`Option`] of aggregated bootstrapping key.
    pub fn bs_key(&self) -> Option<&PhantomBsKey> {
        self.bs_key.as_ref()
    }

    /// Aggregates round 1 keys and sets the aggregated public key.
    pub fn aggregate_round_1_keys<'a>(
        &mut self,
        round_1_keys: impl IntoIterator<Item = &'a PhantomRound1Key>,
    ) {
        self.ops.aggregate_pk_shares(round_1_keys);
    }

    /// Aggregates round 1 keys and sets the aggregated ring-packing key and
    /// bootstrapping key.
    pub fn aggregate_round_2_keys<'a>(
        &mut self,
        round_2_keys: impl IntoIterator<Item = &'a PhantomRound2Key>,
    ) {
        let (rp_key_shares, bs_key_shares): (Vec<_>, Vec<_>) = round_2_keys
            .into_iter()
            .map(|key| (&key.rp_key_share, &key.bs_key_share))
            .unzip();
        let rp_key = self.ops.aggregate_rp_key_shares(rp_key_shares);
        self.set_rp_key(rp_key);
        let bs_key = self.ops.aggregate_bs_key_shares(bs_key_shares);
        self.set_bs_key(bs_key);
    }

    // Sets the aggregated public key.
    pub fn set_pk(&mut self, pk: PhantomPk) {
        self.ops.set_pk(pk);
    }

    /// Sets the aggregated ring-packing key.
    pub fn set_rp_key(&mut self, rp_key: PhantomRpKey) {
        let mut rp_key_prep = PhantomRpKeyPrep::allocate_eval(
            self.ops.ring_packing_param(),
            self.ops.ring_rp().eval_size(),
        );
        prepare_rp_key(self.ops.ring_rp(), &mut rp_key_prep, &rp_key);
        self.rp_key = Some(rp_key);
        self.rp_key_prep = Some(rp_key_prep);
    }

    /// Sets the aggregated bootstrapping key.
    pub fn set_bs_key(&mut self, bs_key: PhantomBsKey) {
        let ring: EvaluationRing = RingOps::new(self.ops.param.modulus, self.ops.param.ring_size);
        let mut bs_key_prep = PhantomBsKeyPrep::allocate_eval(**self.ops.param, ring.eval_size());
        prepare_bs_key(&ring, &mut bs_key_prep, &bs_key);
        self.bs_key = Some(bs_key);
        self.evaluator = Some(Arc::new(FhewBoolEvaluator::new(bs_key_prep)));
    }

    /// Encrypts bits in batch.
    ///
    /// # Panics
    ///
    /// Panics if [`PhantomEvaluator::set_pk`] is not called yet.
    pub fn batched_pk_encrypt(&self, ms: impl IntoIterator<Item = bool>) -> PhantomBatchedCt {
        self.ops.batched_pk_encrypt(ms)
    }

    /// Unbatchs ct generated by [`PhantomUser::batched_pk_encrypt`] or
    /// [`PhantomEvaluator::batched_pk_encrypt`], and wrap them into
    /// [`PhantomBool`] for further computation.
    pub fn unbatch(&self, ct_batched: &PhantomBatchedCt) -> Vec<PhantomBool> {
        ct_batched
            .extract_all(self.ops.ring())
            .into_iter()
            .map(|ct| PhantomBool::new(Arc::clone(self.evaluator.as_ref().unwrap()), ct))
            .collect()
    }

    /// Packs [`PhantomBool`]s into [`PhantomPackedCt`] for decryption.
    pub fn pack<'a>(&self, cts: impl IntoIterator<Item = &'a PhantomBool>) -> PhantomPackedCt {
        PhantomPackedCt(FhewBoolPackedCiphertext::pack_ms(
            self.ops.ring(),
            self.ops.ring_rp(),
            self.rp_key_prep.as_ref().unwrap(),
            cts.into_iter().map(PhantomBool::ct),
        ))
    }

    /// Aggregates decryption share and returns decrypted bits.
    pub fn aggregate_dec_shares(
        &self,
        ct_packed: &PhantomPackedCt,
        dec_shares: Vec<PhantomPackedCtDecShare>,
    ) -> Vec<bool> {
        self.ops.aggregate_dec_shares(ct_packed, &dec_shares)
    }
}

type Ring = NativeRing;

type EvaluationRing = NoisyNativeRing;

type KeySwitchMod = NonNativePowerOfTwo;

type PackingRing = PrimeRing;

pub type PhantomRpKey = RingPackingKeyOwned<Elem<PackingRing>>;

pub type PhantomRpKeyPrep = RingPackingKeyOwned<<PackingRing as RingOps>::EvalPrep>;

pub type PhantomBsKey = FhewBoolKeyOwned<Elem<Ring>, Elem<KeySwitchMod>>;

pub type PhantomBsKeyPrep =
    FhewBoolKeyOwned<<EvaluationRing as RingOps>::EvalPrep, Elem<KeySwitchMod>>;

pub type PhantomCt = FhewBoolCiphertextOwned<Elem<Ring>>;

pub type PhantomBool = FheBool<Arc<FhewBoolEvaluator<EvaluationRing, KeySwitchMod>>>;

wasm_bindgen_wrapper!(
    PhantomSk(RlweSecretKeyOwned<i64>),
    PhantomSkKs(LweSecretKeyOwned<i64>),
    PhantomPkShare(SeededRlwePublicKeyOwned<Elem<Ring>>),
    PhantomRpKeyShare(RingPackingKeyShareOwned<Elem<PackingRing>>),
    PhantomBsKeyShare(FhewBoolMpiKeyShareOwned<Elem<Ring>, Elem<KeySwitchMod>>),
    PhantomPk(RlwePublicKeyOwned<Elem<Ring>>),
    PhantomBatchedCt(FhewBoolBatchedCiphertextOwned<Elem<Ring>>),
    PhantomPackedCt(FhewBoolPackedCiphertextOwned<Elem<PackingRing>>),
    PhantomPackedCtDecShare(RlweDecryptionShareListOwned<Elem<Ring>>),
);

#[derive(Clone, Debug)]
pub struct PhantomOps {
    param: PhantomParam,
    ring: NativeRing,
    mod_ks: NonNativePowerOfTwo,
    ring_rp: PrimeRing,
    pk: Option<PhantomPk>,
}

impl PhantomOps {
    pub fn new(param: PhantomParam) -> Self {
        Self {
            param,
            ring: RingOps::new(param.modulus, param.ring_size),
            mod_ks: ModulusOps::new(param.lwe_modulus),
            ring_rp: RingOps::new(param.ring_packing_modulus.unwrap(), param.ring_size),
            pk: None,
        }
    }

    pub fn param(&self) -> &PhantomParam {
        &self.param
    }

    fn crs(&self) -> &PhantomCrs {
        &self.param.crs
    }

    fn fhew_param(&self) -> &FhewBoolParam {
        self.param()
    }

    fn ring_packing_param(&self) -> RingPackingParam {
        RingPackingParam {
            modulus: self
                .param()
                .ring_packing_modulus
                .unwrap_or_else(|| self.param().modulus),
            ring_size: self.param().ring_size,
            sk_distribution: self.param().sk_distribution,
            noise_distribution: self.param().noise_distribution,
            auto_decomposition_param: self.param().ring_packing_auto_decomposition_param,
        }
    }

    fn ring(&self) -> &Ring {
        &self.ring
    }

    fn mod_ks(&self) -> &KeySwitchMod {
        &self.mod_ks
    }

    fn ring_rp(&self) -> &PackingRing {
        &self.ring_rp
    }

    fn sk_gen(&self, mut rng: StdRng) -> PhantomSk {
        PhantomSk(RlweSecretKey::sample(
            self.param().ring_size,
            self.param().sk_distribution,
            &mut rng,
        ))
    }

    fn sk_ks_gen(&self, mut rng: StdRng) -> PhantomSkKs {
        PhantomSkKs(LweSecretKey::sample(
            self.param().lwe_dimension,
            self.param().lwe_sk_distribution,
            &mut rng,
        ))
    }

    fn pk_share_gen(&self, sk: &PhantomSk, mut rng: StdRng) -> PhantomPkShare {
        let mut pk = SeededRlwePublicKey::allocate(self.param().ring_size);
        pk_share_gen(
            self.ring(),
            &mut pk,
            self.param(),
            &self.crs().fhew(),
            sk.as_view(),
            &mut rng,
        );
        PhantomPkShare(pk)
    }

    fn rp_key_share_gen(&self, sk: &PhantomSk, mut rng: StdRng) -> PhantomRpKeyShare {
        let mut rp_key = RingPackingKeyShareOwned::allocate(self.ring_packing_param());
        rp_key_share_gen(
            self.ring_rp(),
            &mut rp_key,
            &self.crs().ring_packing(),
            sk.as_view(),
            &mut rng,
        );
        PhantomRpKeyShare(rp_key)
    }

    fn bs_key_share_gen(
        &self,
        share_idx: usize,
        sk: &PhantomSk,
        sk_ks: &PhantomSkKs,
        mut rng: StdRng,
    ) -> PhantomBsKeyShare {
        let mut bs_key_share = FhewBoolMpiKeyShareOwned::allocate(**self.param(), share_idx);
        bs_key_share_gen(
            self.ring(),
            self.mod_ks(),
            &mut bs_key_share,
            &self.crs().fhew(),
            sk.as_view(),
            self.pk.as_deref().unwrap(),
            sk_ks.as_view(),
            &mut rng,
        );
        PhantomBsKeyShare(bs_key_share)
    }

    fn aggregate_pk_shares<'a>(&mut self, pk_shares: impl IntoIterator<Item = &'a PhantomPkShare>) {
        let mut pk = RlwePublicKey::allocate(self.fhew_param().ring_size);
        aggregate_pk_shares(
            self.ring(),
            &mut pk,
            &self.crs().fhew(),
            pk_shares.into_iter().map(|wrapper| &wrapper.0),
        );
        self.set_pk(PhantomPk(pk));
    }

    fn set_pk(&mut self, pk: PhantomPk) {
        self.pk = Some(pk);
    }

    fn aggregate_rp_key_shares<'a>(
        &mut self,
        rp_key_shares: impl IntoIterator<Item = &'a PhantomRpKeyShare>,
    ) -> PhantomRpKey {
        let mut rp_key = PhantomRpKey::allocate(self.ring_packing_param());
        aggregate_rp_key_shares(
            self.ring_rp(),
            &mut rp_key,
            &self.crs().ring_packing(),
            rp_key_shares.into_iter().map(|wrapper| &wrapper.0),
        );
        rp_key
    }

    fn aggregate_bs_key_shares<'a>(
        &mut self,
        bs_key_shares: impl IntoIterator<Item = &'a PhantomBsKeyShare>,
    ) -> PhantomBsKey {
        let mut bs_key = PhantomBsKey::allocate(*self.fhew_param());
        aggregate_bs_key_shares(
            self.ring(),
            self.mod_ks(),
            &mut bs_key,
            &self.crs().fhew(),
            bs_key_shares.into_iter().map(|wrapper| &wrapper.0),
        );
        bs_key
    }

    fn batched_pk_encrypt(&self, ms: impl IntoIterator<Item = bool>) -> PhantomBatchedCt {
        PhantomBatchedCt(FhewBoolBatchedCiphertext::pk_encrypt(
            self.fhew_param(),
            self.ring(),
            self.pk.as_deref().unwrap(),
            ms,
            &mut LweRng::new(StdRng::from_entropy(), StdRng::from_entropy()),
        ))
    }

    fn decrypt_share(&self, sk: &PhantomSk, ct: &PhantomPackedCt) -> PhantomPackedCtDecShare {
        PhantomPackedCtDecShare(ct.decrypt_share(
            &self.param,
            self.ring_rp(),
            sk.as_view(),
            &mut LweRng::new(StdRng::from_entropy(), StdRng::from_entropy()),
        ))
    }

    fn aggregate_dec_shares<'a>(
        &self,
        ct: &PhantomPackedCt,
        dec_shares: impl IntoIterator<Item = &'a PhantomPackedCtDecShare>,
    ) -> Vec<bool> {
        ct.aggregate_decryption_shares(
            self.ring_rp(),
            dec_shares.into_iter().map(|wrapper| &wrapper.0),
        )
    }
}

macro_rules! wasm_bindgen_wrapper {
    (@ $outer:ident($inner:ty)) => {
        #[wasm_bindgen]
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct $outer($inner);

        impl core::ops::Deref for $outer {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $outer {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$inner> for $outer {
            fn from(inner: $inner) -> $outer {
                $outer(inner)
            }
        }

        impl From<$outer> for $inner {
            fn from(outer: $outer) -> $inner {
                outer.0
            }
        }
    };
    ($($outer:ident($inner:ty),)*) => {
        $(wasm_bindgen_wrapper!(@ $outer($inner));)*
    }
}

use wasm_bindgen_wrapper;
