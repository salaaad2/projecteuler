use Vec;
// sum of all multiples of 3 or 5 from 1 to max
fn mult_three_or_five(max: i32) -> i32
{
    let mut ret: i32 = 0;
    for i in 1..max {
        if i % 3 == 0 || i % 5 == 0
        {
            ret += i;
        }
    }
    ret
}

// return the sum of all even numbers in the fibonacci sequence, below max (4.000.000)
fn even_fibonacci(max: i32) -> i32
{
    let mut ret: i32 = 0;
    let mut sum: i32 = 0;
    let mut prev: i32 = 1;
    let mut prev_1: i32 = 0;
    while ret <= max {
        ret = prev + prev_1;
        prev_1 = prev;
        prev = ret;
        if ret & 1 == 0
        {
            sum += ret;
        }
    }
    sum
}

// largest prime factor of 600851475143
//  a prime factor is a prime number that can be used along with other
//  prime numbers in a multiplication to end up with the number provided
//  return the largest prime for n
fn largest_prime_factor(n: u64) -> u64
{
    let mut i: u64 = 2;
    let mut n_copy = n;
    while i * i < n_copy
    {
        while n_copy % i == 0
        {
            n_copy = n_copy / i;
        }
        i += 1;
    }
    n_copy
}

fn main() {
    println!("{}", mult_three_or_five(1000));
    println!("{}", even_fibonacci(4000000));
    println!("{}", largest_prime_factor(13195));
    println!("{}", largest_prime_factor(600851475143));
}
