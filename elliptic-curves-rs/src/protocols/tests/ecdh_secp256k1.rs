use crate::curves::secp256k1::secp256k1::Secp256k1Curve;
use crate::protocols::ecdh::*;

#[test]
fn test_ecdh_secp256k1() {
    let (alice_private_key, alice_public_key) = 
        Ecdh::<Secp256k1Curve>::generate_keypair();
    let (bob_private_key, bob_public_key) = 
        Ecdh::<Secp256k1Curve>::generate_keypair();

    // Alice's and Bob's keys should be different
    assert_ne!(alice_public_key, bob_public_key);
    assert_ne!(alice_private_key, bob_private_key);

    // Ecdh 구조체에서 어떤 구체적인 타입을 사용할지 :: 를 이용해 명시적으로 알려줌
    let shared_alice = Ecdh::<Secp256k1Curve>::compute_shared_secret(
                                &alice_private_key, &bob_public_key);
    let shared_bob = Ecdh::<Secp256k1Curve>::compute_shared_secret(
                                &bob_private_key, &alice_public_key);

    assert_eq!(shared_alice, shared_bob);
}
