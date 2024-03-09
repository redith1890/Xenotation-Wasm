use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
#[wasm_bindgen]
pub fn represent_power_of_two(mut n: u32) -> String {
    let mut i = 0;

    while n > 1 {
        n /= 2;
        i += 1;
    }
    let doc = '.';
    let docs: String = doc.to_string().repeat(i as usize);
    docs
}
#[wasm_bindgen]
pub fn find_primes_up_to(n: u32) -> js_sys::Uint32Array {
    let mut sieve = vec![true; (n + 1) as usize];
    let mut p = 2;
    while p * p <= n {
        if sieve[p as usize] == true {
            let mut i = p * p;
            while i <= n {
                sieve[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }
    let sieve = sieve
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .skip(2)
        .map(|(i, _)| i as u32)
        .collect::<Vec<u32>>();
    let sieve = js_sys::Uint32Array::from(sieve.as_slice());
    return sieve;
}
#[wasm_bindgen]
pub fn represent_prime(prime: u32, primes: &js_sys::Uint32Array) -> String {
    let primes = primes.to_vec();
    let index = primes.iter().position(|&x| x == prime).unwrap() + 1;
    if index == 1 {
        return ".".to_string();
    }
    let primes = js_sys::Uint32Array::from(primes.as_slice());
    return format!("({})", represent_number(index as u32, &primes));
}
#[wasm_bindgen]
pub fn represent_number(n: u32, primes: &js_sys::Uint32Array) -> String {
    let primes = primes.to_vec();
    if is_power_of_two(n) {
        return represent_power_of_two(n);
    } else if primes.contains(&n) {
        let primes = js_sys::Uint32Array::from(primes.as_slice());
        return represent_prime(n, &primes);
    } else {
        let factors = descompose_into_primes(n);
        let primes = js_sys::Uint32Array::from(primes.as_slice());
        return factors
            .iter()
            .map(|x| represent_number(*x, &primes))
            .collect::<String>();
    }
}
#[wasm_bindgen]
pub fn descompose_into_primes(n: u32) -> Vec<u32> {
    let mut i = 2;
    let mut factors = vec![];
    let mut n = n;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
        } else {
            n /= i;
            factors.push(i);
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}
#[wasm_bindgen]
pub fn representation(number:u32) -> String {
    let maximum_number_for_primes = if number < 50 {100} else {number*2};
    let primes_up_to_maximum = find_primes_up_to(maximum_number_for_primes);
    let representation = represent_number(number, &primes_up_to_maximum);
    representation
}