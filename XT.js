
// Check if a number is a power of two
function is_power_of_two(n) {
    return n !== 0 && (n & (n - 1)) === 0;
}

// Represent a power of two with dots
function represent_power_of_two(n) {
    let count = 0;
    while (n > 1) {
        n = Math.floor(n / 2);
        count += 1;
    }
    return '.'.repeat(count);
}

// Find all prime numbers up to a certain number (Sieve of Eratosthenes)
function find_primes_up_to(n) {
    let sieve = new Array(n + 1).fill(true);
    let p = 2;
    while (p * p <= n) {
        if (sieve[p]) {
            for (let i = p * p; i <= n; i += p) {
                sieve[i] = false;
            }
        }
        p += 1;
    }
    let primes = [];
    for (let i = 2; i <= n; i++) {
        if (sieve[i]) {
            primes.push(i);
        }
    }
    return primes;
}

// Represent a prime number based on its index in the list of primes
function represent_prime(prime, primes) {
    let index = primes.indexOf(prime) + 1;
    if (index === 1) {
        return '.';
    }
    return '(' + represent_number(index, primes) + ')';
}

// Represent a number as a power of two, a prime, or decomposed into prime factors
function represent_number(n, primes) {
    if (is_power_of_two(n)) {
        return represent_power_of_two(n);
    } else if (primes.includes(n)) {
        return represent_prime(n, primes);
    } else {
        let factors = decompose_into_primes(n, primes);
        return factors.map(factor => represent_prime(factor, primes)).join('');
    }
}

// Decompose a number into its prime factors
function decompose_into_primes(n, primes) {
    let factors = [];
    for (let i = 0; primes[i] * primes[i] <= n; i++) {
        while (n % primes[i] === 0) {
            factors.push(primes[i]);
            n /= primes[i];
        }
    }
    if (n > 1) {
        factors.push(n);
    }
    return factors;
}
