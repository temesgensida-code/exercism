pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..(n+1){
        sum+=i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..(n+1){
        sum += i * i;
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
