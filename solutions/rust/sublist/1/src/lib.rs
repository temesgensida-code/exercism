#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T:PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    let is_contained = |sub:&[T],super_list:&[T]|{
        super_list
            .windows(sub.len())
            .any(|window| window == sub)
    };
    match(first_list, second_list){
        (l,r) if l==r => Equal,
        (l,r) if l.is_empty() || is_contained(l, r) => Sublist,
        (l,r) if r.is_empty() || is_contained(r, l) => Superlist,
        _ => Unequal,
    }
    
}

