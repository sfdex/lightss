use rand::prelude::*;

const PASSWORD_LENGTH: u32 = 256;

type Password = [u8; 256];
pub type EncodePassword = Password;
pub type DecodePassword = Password;

pub fn gen_passwd() -> (EncodePassword, DecodePassword) {
    let mut password: Vec<u8> = (0..=255).collect();
    let mut rng = thread_rng();
    password.shuffle(&mut rng);

    for i in 0..256 {
        for j in 0..256 {
            if i == j { continue; }
            if password[i] == password[j] {
                panic!("repeat: ({i})=({j})={:?}", password[i])
            }
        }
    }

    println!("passwd: {:?}", password);

    let mut e: EncodePassword = [0; 256];
    let mut d: DecodePassword = [0; 256];

    for i in 0..256 {
        let k = password[i];
        e[i] = k;
        d[k as usize] = i as u8;
    }

    (e, d)
}