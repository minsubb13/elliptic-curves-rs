use crate::curves::secp256k1::secp256k1::Secp256k1Curve;
use crate::curves::secp256k1::secp256k1::FrSecp256k1;
use crate::protocols::ecdh::*;
use crate::protocols::ecdsa::*;

use ark_ff::PrimeField;
use sha2::{Sha256, Digest};

#[test]
fn test_ecdsa_secp256k1() {
    let (alice_private_key, alice_public_key) =
        Ecdh::<Secp256k1Curve>::generate_keypair();
    let (bob_private_key, bob_public_key) =
        Ecdh::<Secp256k1Curve>::generate_keypair();
    
    // Alice's and Bob's keys should be different
    assert_ne!(alice_public_key, bob_public_key);
    assert_ne!(alice_private_key, bob_private_key);

    let digest = Sha256::digest(b"hello world");
    let z: FrSecp256k1 = FrSecp256k1::from_be_bytes_mod_order(&digest);
    let wrong_digest = Sha256::digest(b"hello there");
    let wrong_z = FrSecp256k1::from_be_bytes_mod_order(&wrong_digest);

    let signature =
        Ecdsa::<Secp256k1Curve>::signing_message(&alice_private_key, z);
    
    assert!(Ecdsa::<Secp256k1Curve>::verifying_message(
        &alice_public_key, z, &signature
    ));
    assert!(!Ecdsa::<Secp256k1Curve>::verifying_message(
        &alice_public_key, wrong_z, &signature
    ));
    assert!(!Ecdsa::<Secp256k1Curve>::verifying_message(
        &bob_public_key, z, &signature
    ));
}
