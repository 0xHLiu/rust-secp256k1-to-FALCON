extern crate secp256k1;
use falcon_rust::falcon512;
use rand::thread_rng;
use rand::Rng;

use secp256k1::{PublicKey, Secp256k1, SecretKey};

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
    // println!("{:?}", thread_rng().gen());

    // let (falcon_seckey, falcon_pubkey) = falcon512::keygen(thread_rng().gen());

    // println!("Falcon Secret Key: {:?}", falcon_seckey);
    // println!("Falocn Public Key: {:?}", falcon_pubkey);

    // Second option:
    let seckey = SecretKey::new(&mut rng);
    let _pubkey = PublicKey::from_secret_key(&secp, &seckey);
}
