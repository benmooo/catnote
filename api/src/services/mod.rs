mod email;
mod hash;
mod jwt;

pub use email::*;
pub use hash::*;
pub use jwt::*;

use rand::{distributions::Alphanumeric, Rng};
pub fn rand_str(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
