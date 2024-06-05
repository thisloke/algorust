use std::sync::{LazyLock, Mutex};
use rand::Rng;

pub static VECTORS: LazyLock<Mutex<[Vec<i32>; 32]>> = LazyLock::new(|| Mutex::new(get_vectors()));

pub fn get_vectors() -> [Vec<i32>; 32] {
    let mut vectors: [Vec<i32>; 32] = Default::default();
    for i in 0..32 {
        let vector_size = rand::thread_rng().gen_range(0..100);

        for _j in 0..vector_size {
            vectors[i].push(rand::thread_rng().gen_range(-1000..1000))
        }
    }
    vectors
}

