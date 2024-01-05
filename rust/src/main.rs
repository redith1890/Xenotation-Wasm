use std::io::{self, Write};
use std::time::Instant;

fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
        
}

fn represent_power_of_two(n: &u32) -> String {
    let mut i = 0;
    let mut n = *n;
    while n > 1{
        n /= 2;
        i += 1;
    }
    let doc = '.';
    let docs: String = doc.to_string().repeat(i as usize);
    docs
}

fn find_primes_up_to(n: u32) -> Vec<u32> {
    let mut sieve = vec![true; (n+1) as usize];
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
    return sieve.iter().enumerate().filter(|(_, &v)| v).skip(2).map(|(i, _)| i as u32).collect::<Vec<u32>>();
}

fn represent_prime(prime:u32, primes:&Vec<u32>) -> String {
    let index = primes.iter().position(|&x| x == prime).unwrap() +1;
    if index == 1 {
        return ".".to_string();}
    return format!("({})", represent_number(index as u32, &primes));
}

fn represent_number(n: u32, primes: &Vec<u32>) -> String {
    if is_power_of_two(n) {
        return represent_power_of_two(&n);
    }
    else if primes.contains(&n)  {
        return represent_prime(n, primes);
    }
    else {
        let factors = descompose_into_primes(n);
        return factors.iter().map(|x| represent_number(*x, primes)).collect::<String>();
    }
    
}

fn descompose_into_primes(n: u32) -> Vec<u32> {
    let mut i = 2;
    let mut factors = vec![];
    let mut n = n;
    while i*i <= n {
        if n % i != 0 {
            i += 1;
        }
        else {
            n /= i;
            factors.push(i);
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors

}

fn main() {
    println!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    let start_time = Instant::now();
    let number: u32 = number.trim().parse().unwrap();
    let maximum_number_for_primes = if number < 50 {100} else {number*2};
    let primes_up_to_maximum = find_primes_up_to(maximum_number_for_primes);
    let representation = represent_number(number, &primes_up_to_maximum);
    println!("{}", representation);
    let elapsed_time = start_time.elapsed();
    
    let elapsed_ms = elapsed_time.as_millis();
    println!("Execution time: {} milliseconds", elapsed_ms);
}






#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_is_power_of_two(){

        //assert_eq!(is_power_of_two(0), false);
        //assert_eq!(is_power_of_two(1), false);
        assert_eq!(is_power_of_two(2), true);
        assert_eq!(is_power_of_two(3), false);
        assert_eq!(is_power_of_two(4), true);
        assert_eq!(is_power_of_two(5), false);
        assert_eq!(is_power_of_two(6), false);
        assert_eq!(is_power_of_two(7), false);
        assert_eq!(is_power_of_two(8), true);
        assert_eq!(is_power_of_two(9), false);
        assert_eq!(is_power_of_two(10), false);
        assert_eq!(is_power_of_two(11), false);
        assert_eq!(is_power_of_two(12), false);
        assert_eq!(is_power_of_two(13), false);
        assert_eq!(is_power_of_two(14), false);
        assert_eq!(is_power_of_two(15), false);
        assert_eq!(is_power_of_two(16), true);
        assert_eq!(is_power_of_two(17), false);
        assert_eq!(is_power_of_two(18), false);
        assert_eq!(is_power_of_two(19), false);
        assert_eq!(is_power_of_two(20), false);
        assert_eq!(is_power_of_two(21), false);
        assert_eq!(is_power_of_two(22), false);
        assert_eq!(is_power_of_two(23), false);
        assert_eq!(is_power_of_two(24), false);
        assert_eq!(is_power_of_two(25), false);
        assert_eq!(is_power_of_two(26), false);
        assert_eq!(is_power_of_two(27), false);
        assert_eq!(is_power_of_two(28), false);
        assert_eq!(is_power_of_two(29), false);
        assert_eq!(is_power_of_two(30), false);
        assert_eq!(is_power_of_two(31), false);
        assert_eq!(is_power_of_two(32), true);
        assert_eq!(is_power_of_two(33), false);
        assert_eq!(is_power_of_two(34), false);
        assert_eq!(is_power_of_two(35), false);
        assert_eq!(is_power_of_two(36), false);
        assert_eq!(is_power_of_two(37), false);
}
    #[test]
    fn test_represent_power_of_two(){
        assert_eq!(represent_power_of_two(&0), "".to_string());
        assert_eq!(represent_power_of_two(&1), "".to_string());
        assert_eq!(represent_power_of_two(&2), ".".to_string());
        assert_eq!(represent_power_of_two(&3), ".".to_string());
        assert_eq!(represent_power_of_two(&4), "..".to_string());
        assert_eq!(represent_power_of_two(&5), ".".to_string());
        assert_eq!(represent_power_of_two(&6), "..".to_string());
        assert_eq!(represent_power_of_two(&7), "..".to_string());
        assert_eq!(represent_power_of_two(&8), "...".to_string());
        assert_eq!(represent_power_of_two(&9), "..".to_string());
        assert_eq!(represent_power_of_two(&10), "..".to_string());
        assert_eq!(represent_power_of_two(&11), ".".to_string());
        assert_eq!(represent_power_of_two(&12), "...".to_string());
        assert_eq!(represent_power_of_two(&13), "".to_string());
        assert_eq!(represent_power_of_two(&14), "".to_string());
        assert_eq!(represent_power_of_two(&15), "".to_string());
        assert_eq!(represent_power_of_two(&16), "....".to_string());
        assert_eq!(represent_power_of_two(&17), "".to_string());
        assert_eq!(represent_power_of_two(&18), "".to_string());
        assert_eq!(represent_power_of_two(&19), "".to_string());
        assert_eq!(represent_power_of_two(&20), "".to_string());
        assert_eq!(represent_power_of_two(&21), "".to_string());
        assert_eq!(represent_power_of_two(&22), "".to_string());
        assert_eq!(represent_power_of_two(&23), "".to_string());
        assert_eq!(represent_power_of_two(&24), "".to_string());
        assert_eq!(represent_power_of_two(&25), "".to_string());
        assert_eq!(represent_power_of_two(&26), "".to_string());
        assert_eq!(represent_power_of_two(&27), "".to_string());
        assert_eq!(represent_power_of_two(&28), "".to_string());
        assert_eq!(represent_power_of_two(&29), "".to_string());
        assert_eq!(represent_power_of_two(&30), "".to_string());
        assert_eq!(represent_power_of_two(&31), "".to_string());
    }
    #[test]
    fn test_primes_up_to() {
        assert_eq!(find_primes_up_to(2), vec![2]);
        assert_eq!(find_primes_up_to(3), vec![2, 3]);
        assert_eq!(find_primes_up_to(4), vec![2, 3]);
        assert_eq!(find_primes_up_to(5), vec![2, 3, 5]);
        assert_eq!(find_primes_up_to(6), vec![2, 3, 5]);
        assert_eq!(find_primes_up_to(7), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_up_to(8), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_up_to(9), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_up_to(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(find_primes_up_to(12), vec![2, 3, 5, 7, 11]);
        assert_eq!(find_primes_up_to(13), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_up_to(14), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_up_to(15), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_up_to(16), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_up_to(17), vec![2, 3, 5, 7, 11, 13, 17]);
        assert_eq!(find_primes_up_to(18), vec![2, 3, 5, 7, 11, 13, 17]);
    }


    #[test]
    fn test_represent_prime(){
        let primes = find_primes_up_to(100);
        assert_eq!(represent_prime(2, &primes), ".".to_string());
        assert_eq!(represent_prime(3, &primes), "(.)".to_string());
        assert_eq!(represent_prime(5, &primes), "((.))".to_string());
        assert_eq!(represent_prime(7, &primes), "(..)".to_string());
        assert_eq!(represent_prime(11, &primes), "(((.)))".to_string());
        assert_eq!(represent_prime(13, &primes), "(.(.))".to_string());
        assert_eq!(represent_prime(17, &primes), "((..))".to_string());
        assert_eq!(represent_prime(19, &primes), "(...)".to_string());
        assert_eq!(represent_prime(23, &primes), "((.)(.))".to_string());
    }
}



