pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s){
        panic!("errors..");
    }
    1 << (s-1)
}

pub fn total()->u64 {
    u64::MAX
}