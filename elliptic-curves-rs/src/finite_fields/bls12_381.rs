use ark_ff::{BigInteger, BigInteger256, Fp256, PrimeField};
use ark_ff::fields::{MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787"]
#[generator = "2"]
pub struct Bls12381Config;

pub type Fq = Fp384<MontBackend<Bls12381Config, 6>>;
