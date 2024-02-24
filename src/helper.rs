use rand::distributions::{Alphanumeric, DistString};

pub fn generate_random_string() -> String{
    let password = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    return password
}