use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
pub fn generate_random_string() -> String{
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
    .map(|_| {
    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
    })
    .collect();

    return password

}