pub mod password;

pub fn init() {
    let (encoded_password, decoded_password) = password::gen_passwd();
    let cipher = Cipher::new(encoded_password, decoded_password);

    let plain_text = "Hello World";
    println!("plain   bytes: {:?}",plain_text.as_bytes());

    let encoded = cipher.encode(plain_text.as_bytes());
    println!("encoded bytes: {:?}", encoded);

    let decoded = cipher.decode(&encoded);
    println!("decoded bytes: {:?}", decoded);

    let decoded_text = String::from_utf8(decoded).unwrap();
    println!("decoded_text = {decoded_text}")
}

#[derive(Copy)]
pub struct Cipher {
    encode_password: password::EncodePassword,
    decode_password: password::DecodePassword,
}

impl Clone for Cipher{
    fn clone(&self) -> Self {
        // Self{
        //     encode_password: *&self.encode_password,
        //     decode_password: *&self.decode_password,
        // }

        *self
    }
}

impl Cipher {
    pub fn new_symmetric(encode_password: password::Password) -> Self {
        let mut decode_password:[u8;256] = [0;256];
        for i in 0..256usize {
            let v = encode_password[i];
            decode_password[v as usize] = i as u8;
        }
        Self { encode_password, decode_password }
    }

    pub fn new(encode_password: password::EncodePassword, decode_password: password::DecodePassword) -> Self {
        Self { encode_password, decode_password }
    }

    pub fn encode(&self, plain_bytes: &[u8]) -> Vec<u8> {
        let n = plain_bytes.len();
        let result: Vec<u8> = plain_bytes.iter()
            .map(|p| {
                let index = *p as usize;
                self.encode_password[index]
            }).collect();

        if n != result.len() {
            panic!("encode failed");
        }

        result
    }

    pub fn decode(&self, encrypted_bytes: &[u8]) -> Vec<u8> {
        let n = encrypted_bytes.len();
        let result: Vec<u8> = encrypted_bytes.iter()
            .map(|e| {
                let index = *e as usize;
                self.decode_password[index]
            }).collect();

        if n != result.len() {
            panic!("decode failed");
        }

        result
    }
}