use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut collect=HashSet::new();
    for &factor in factors{
        if factor == 0{
            continue;
        }
        let mut multiples =factor;
        while multiples < limit{
            collect.insert(multiples);
            multiples+=factor;
        }
    }
    collect.iter().sum()

}
