pub fn is_armstrong_number(num: u32) -> bool {
    let digits:u32=num.to_string().len() as u32;
    let mut temp = num;
    let mut sum:u64=0;

    while temp > 0{
        sum += (temp as u64 % 10).pow(digits );
        temp/=10;
    }
    sum == (num as u64)
}
fn main(){
    let num:u32=153;
    println!("{}",is_armstrong_number(num));
}
