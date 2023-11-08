import time

def is_power_of_two(n):
    return n != 0 and ((n & (n - 1)) == 0)

def represent_power_of_two(n):
    count = 0
    while n > 1:
        n //= 2
        count += 1
    return '.' * count

def find_primes_up_to(n):
    sieve = [True] * (n + 1)
    p = 2
    while p * p <= n:
        if sieve[p]:
            for i in range(p * p, n + 1, p):
                sieve[i] = False
        p += 1
    return [p for p in range(2, n+1) if sieve[p]]

def represent_prime(prime, primes):
    index = primes.index(prime) + 1
    if index == 1:
        return '.'
    return '(' + represent_number(index, primes) + ')'

def represent_number(n, primes):
    if is_power_of_two(n):
        return represent_power_of_two(n)
    elif n in primes:
        return represent_prime(n, primes)
    else:
        factors = decompose_into_primes(n)
        return ''.join(represent_prime(factor, primes) for factor in factors)

def decompose_into_primes(n):
    i = 2
    factors = []
    while i * i <= n:
        if n % i:
            i += 1
        else:
            n //= i
            factors.append(i)
    if n > 1:
        factors.append(n)
    return factors

def formatted_time(time_ns):
    if time_ns < 1e3:
        return f"{time_ns} nanoseconds"
    elif time_ns < 1e6:
        return f"{time_ns / 1e3} microseconds"
    elif time_ns < 1e9:
        return f"{time_ns / 1e6} milliseconds"
    else:
        return f"{time_ns / 1e9} seconds"

# Main process
number = int(input("Enter a number: "))
start_time = time.time_ns()
maximum_number_for_primes = max(100, number * 2)
primes_up_to_maximum = find_primes_up_to(maximum_number_for_primes)
representation = represent_number(number, primes_up_to_maximum)
end_time = time.time_ns()

print(f"Representation of the number {number}: {representation}")
print(f"Time elapsed: {formatted_time(end_time - start_time)}")
