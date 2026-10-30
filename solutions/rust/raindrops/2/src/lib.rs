pub fn raindrops(n: u32) -> String {
    let mut value=String::new();
    if n.is_multiple_of(3){
        value.push_str("Pling");
    }
    if n.is_multiple_of(5){
        value.push_str("Plang");
    }
    if n.is_multiple_of(7) {
        value.push_str("Plong")
    }
    if value.is_empty(){
        return n.to_string();
    }
    value

}
