mod ksi_math {
    use std::mem;

    pub fn fast_pow(base: i128, exp: i128) -> i128 {
        let mut result: i128 = 1;
        let mut base: i128 = base;
        let mut exp: i128 = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            exp >>= 1;
            base *= base;
        }
        result
    }

    pub fn fast_pow_f32(base: i128, exp: i128) -> f32 {
        let mut result: f32 = 1.0;
        let mut base: f32 = base as f32;
        let mut exp: i128 = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            exp >>= 1;
            base *= base;
        }
        result
    }

    pub fn fast_pow_f64(base: i128, exp: i128) -> f64 {
        let mut result: f64 = 1.0;
        let mut base: f64 = base as f64;
        let mut exp: i128 = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            exp >>= 1;
            base *= base;
        }
        result
    }

    pub fn fast_pow_mod(base: i128, exp: i128, modulus: i128) -> i128 {
        let mut result: i128 = 1;
        let mut base: i128 = base % modulus;
        let mut exp: i128 = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            exp >>= 1;
            base = (base * base) % modulus;
        }
        result
    }

    pub fn fast_inv_sqrt_f32(num: f32) -> f32 {
        const THREEHALFS: f32 = 1.5f32;
        let x2: f32 = num * 0.5f32;
        let mut i: u32 = unsafe { mem::transmute(num) }; // evil floating point bit level hacking
        i = 0x5f375a86 - (i >> 1); // what the fuck?
        let y: f32 = unsafe { mem::transmute(i) };

        let y: f32 = y * (THREEHALFS - (x2 * y * y)); // 1st iteration

        // y  = y * ( threehalfs - ( x2 * y * y ) );       // 2nd iteration, this can be removed

        return y;
    }

    pub fn fast_inv_sqrt_f64(num: f64) -> f64 {
        const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
        const THREEHALFS: f64 = 1.5;
        let x2: f64 = num * 0.5;
        let i: u64 = MAGIC_U64 - (unsafe { mem::transmute::<_, u64>(num) } >> 1);
        let y: f64 = unsafe { mem::transmute(i) };

        y * (THREEHALFS - (x2 * y * y))
    }
}

fn main() {
    println!("i128 2^10 = {}", ksi_math::fast_pow(2, 32));
    println!("f32 2^10 = {}", ksi_math::fast_pow_f32(2, 64));
    println!("f64 2^10 = {}", ksi_math::fast_pow_f64(2, 64));
    println!("f64 2^10 = {}", ksi_math::fast_pow_f64(2, 256));

    println!("2^10 mod 7 = {}", ksi_math::fast_pow_mod(2, 10, 7));

    println!("1/sqrt(4) = {}", ksi_math::fast_inv_sqrt_f32(4.0));
    println!("1/sqrt(4) = {}", ksi_math::fast_inv_sqrt_f64(4.0));
}
