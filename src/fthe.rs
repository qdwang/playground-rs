use std::fs;

use tfhe::{prelude::*, ClientKey, FheUint3, CompressedFheUint32, CompressedFheUint8};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8};

fn dump() {
    // let mut ck_dump = vec![];
    // let mut sk_dump = vec![];
    // bincode::serialize_into(&mut ck_dump, &ck).unwrap();
    // bincode::serialize_into(&mut sk_dump, &sk).unwrap();
    // fs::write("ck_dump", ck_dump).unwrap();
    // fs::write("sk_dump", sk_dump).unwrap();
}
fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    let ck = ClientKey::generate(config);
    let sk = ck.generate_server_key();

    let a = FheUint32::encrypt(123u8, &ck);
    let b = FheUint32::encrypt(1234u32, &ck);
    // let dump = bincode::serialize(&a).unwrap();
    // println!("{}", dump.len());

    set_server_key(sk);
    let result = (a + b) / 2;

    let q : u32 = result.decrypt(&ck);
    println!("{q}");
}

fn process() {
    // Basic configuration to use homomorphic integers
    // let config = ConfigBuilder::build(ConfigBuilder::all_enabled());
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();


    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 1344u32;
    let clear_b = 5u32;
    let clear_c = 7u8;

    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    let mut encrypted_a = FheUint32::try_encrypt(clear_a, &client_key).unwrap();
    let encrypted_b = FheUint32::try_encrypt(clear_b, &client_key).unwrap();

    // FheUint8: Encrypted equivalent to u8
    let encrypted_c = FheUint8::try_encrypt(clear_c, &client_key).unwrap();

    // On the server side:
    set_server_key(server_keys);

    // Clear equivalent computations: 1344 * 5 = 6720
    let encrypted_res_mul = &encrypted_a * &encrypted_b;

    // Clear equivalent computations: 1344 >> 5 = 42
    encrypted_a = &encrypted_res_mul >> &encrypted_b;

    // Clear equivalent computations: let casted_a = a as u8;
    let casted_a: FheUint8 = encrypted_a.cast_into();

    // Clear equivalent computations: min(42, 7) = 7
    let encrypted_res_min = &casted_a.min(&encrypted_c);

    // Operation between clear and encrypted data:
    // Clear equivalent computations: 7 & 1 = 1
    let encrypted_res = encrypted_res_min & 1_u8;

    // Decrypting on the client side:
    let clear_res: u8 = encrypted_res.decrypt(&client_key);
    assert_eq!(clear_res, 1_u8);
}
