pub fn raindrops(n: u32) -> String {
    let mut value=String::new();
    if n%3 == 0{
        value.push_str("Pling");
    }
    if n%5 == 0 {
        value.push_str("Plang");
    }
    if n%7 ==0 {
        value.push_str("Plong")
    }
    if value.is_empty(){
        return n.to_string();
    }
    value

}
