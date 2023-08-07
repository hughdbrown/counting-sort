use std::time::{SystemTime, UNIX_EPOCH};

use crate::customer::{
    Customer,
};

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_usize(&mut self, min: usize, max: usize) -> usize {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as usize;
    }

}

pub fn make_random_vec(count: usize, min: usize, max: usize) -> Vec<usize> {
    let mut prng = Prng::new();
    let mut result: Vec::<usize> = Vec::new();
    for _ in 0..count {
        let u = prng.next_usize(min, max);
        result.push(u);
    }
    result
}

pub fn make_random_vec_customer(count: usize, min: usize, max: usize) -> Vec<Customer> {
    let mut prng = Prng::new();
    let mut result: Vec::<Customer> = Vec::new();
    for i in 0..count {
        let v = prng.next_usize(min, max);
        let c: Customer = Customer::new(i, v);
        result.push(c);
    }
    result
}

