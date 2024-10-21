use crate::{
    PhantomBatchedCt, PhantomBool, PhantomEvaluator, PhantomPackedCt, PhantomParam,
    PhantomRound1Key, PhantomRound2Key, PhantomUser,
};
use core::{array::from_fn, iter::repeat_with, ops::*};
use itertools::{izip, Itertools};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn e2e() {
    let param = PhantomParam::I_4P_60;
    let mut server = PhantomEvaluator::new(param);
    let mut users: [PhantomUser; 4] = from_fn(|user_id| {
        let seed = StdRng::from_entropy().gen::<[u8; 32]>().to_vec();
        PhantomUser::new(param, user_id, seed)
    });

    // Initially server doesn't have aggregated public key or bootstrapping key,
    assert!(server.pk().is_none());
    // or bootstrapping key,
    assert!(server.bs_key().is_none());
    // or ring packing key.
    assert!(server.rp_key().is_none());

    /*  Start round 1 key generation (collecting public key shares) */

    // Each user generates round 1 key and submit to server.
    let round_1_keys: [PhantomRound1Key; 4] = users.each_ref().map(|user| user.round_1_key_gen());
    // Server aggregates after collecting all round 1 key shares.
    server.aggregate_round_1_keys(&round_1_keys);
    // Now server has aggregated public key for round 2 key generation.
    let pk = server.pk().cloned().unwrap();

    /*  Start round 2 key generation (collecting bootstrapping key shares) */

    // Each user first retrieves aggregated public key from server,
    users.each_mut().map(|user| user.set_pk(pk.clone()));
    // then generates round 2 key and submit to server.
    let round_2_keys: [PhantomRound2Key; 4] = users.each_ref().map(|user| user.round_2_key_gen());
    // Server aggregates after collecting all round 2 key shares.
    server.aggregate_round_2_keys(&round_2_keys);
    // Now server has bootstrapping key,
    assert!(server.bs_key().is_some());
    // and ring packing key.
    assert!(server.rp_key().is_some());

    /*  Start to do some FHE computation */

    // Each user generates some random bits.
    let inputs: [Vec<bool>; 4] = from_fn(|_| random_bits(10));
    // Each user encrypts bits in batch and submit to server
    let cts_batched: [PhantomBatchedCt; 4] =
        from_fn(|i| users[i].batched_pk_encrypt(inputs[i].clone()));
    // Server extract batched cts from each batch and wrap it as inputs of FHE computation.
    let ct_inputs: [Vec<PhantomBool>; 4] = cts_batched
        .each_ref()
        .map(|ct_batched| server.unbatch(ct_batched));
    // Now we can do FHE computation on these cts, for example XOR 4 users' inputs.
    let ct_outputs: Vec<PhantomBool> = xor_4_bit_vecs(&ct_inputs);
    // Before user gets the outputs, server packs the outputs to save network bandwidth.
    let ct_packed: PhantomPackedCt = server.pack(&ct_outputs);

    /*  Start to collect decryption shares */

    // Each user generates decryption share and submit to server.
    let dec_shares = users.each_ref().map(|user| user.decrypt_share(&ct_packed));
    // Anyone with all the decryption shares can aggregate decryption shares and decrypt.
    let outputs = users[0].aggregate_dec_shares(&ct_packed, dec_shares.to_vec());

    assert_eq!(outputs, xor_4_bit_vecs(&inputs))
}

fn random_bits(n: usize) -> Vec<bool> {
    let mut rng = StdRng::from_entropy();
    repeat_with(|| rng.gen_bool(0.5)).take(n).collect()
}

fn xor_4_bit_vecs<T>(inputs: &[Vec<T>; 4]) -> Vec<T>
where
    T: for<'t> BitOps<&'t T, T>,
    for<'t> &'t T: BitOps<&'t T, T>,
{
    izip!(&inputs[0], &inputs[1], &inputs[2], &inputs[3])
        .map(|(a, b, c, d)| a ^ b ^ c ^ d)
        .collect_vec()
}

trait BitOps<Rhs = Self, Output = Self>:
    BitAnd<Rhs, Output = Output> + BitOr<Rhs, Output = Output> + BitXor<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> BitOps<Rhs, Output> for T where
    T: BitAnd<Rhs, Output = Output> + BitOr<Rhs, Output = Output> + BitXor<Rhs, Output = Output>
{
}
