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


fn main() {
    println!("{}", mult_three_or_five(1000));
}
