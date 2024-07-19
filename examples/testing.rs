#![allow(unused)]

extern crate secp256k1;
use rand::thread_rng;
use rand::Rng;

use secp256k1::{PublicKey, Secp256k1, SecretKey};

use secp256k1::falcon::falcon512;

fn random_32_bytes<R: rand::Rng + ?Sized>(rng: &mut R) -> [u8; 32] {
    let mut ret = [0u8; 32];
    rng.fill(&mut ret);
    ret
}

fn main() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    // First option:
    let (seckey, pubkey) = secp.generate_keypair(&mut rng);

    println!("{:?}", secp);
    println!("Secret Key: {:?}", seckey);
    println!("Public Key: {:?}", pubkey);

    assert_eq!(pubkey, PublicKey::from_secret_key(&secp, &seckey));

    println!("{:?}", rng);
    let mut data = random_32_bytes(&mut rng);
    println!("{:?}", data);

    let random_value: [u8; 32] = thread_rng().gen();
    println!("{:?}", random_value);

    // println!("{:?}", thread_rng().gen());

    let (falcon_seckey, falcon_pubkey) = falcon512::keygen(random_value);

    println!("Falcon Secret Key: {:?}", falcon_seckey);
    println!("Falcon Public Key: {:?}", falcon_pubkey);

    // Second option:
    // let seckey = SecretKey::new(&mut rng);
    // let _pubkey = PublicKey::from_secret_key(&secp, &seckey);
}
