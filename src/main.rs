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
            println!("{}", ret);
            sum += ret;
        }
    }
    sum
}

fn main() {
    println!("{}", mult_three_or_five(1000));
    println!("{}", even_fibonacci(4000000));
}
