pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(start_bottles: u32) -> String {
    let (first, second, num_word) = match start_bottles {
        10 => ("Ten green bottles", "nine green bottles", "One"),
        9 => ("Nine green bottles", "eight green bottles", "two"),
        8 => ("Eight green bottles", "seven green bottles", "three"),
        7 => ("Seven green bottles", "six green bottles", "four"),
        6 => ("Six green bottles", "five green bottles", "five"),
        5 => ("Five green bottles", "four green bottles", "six"),
        4 => ("Four green bottles", "three green bottles", "seven"),
        3 => ("Three green bottles", "two green bottles", "eight"),
        2 => ("Two green bottles", "one green bottle", "nine"),
        1 => ("One green bottle", "no green bottles", "ten"),
        _ => ("", "", ""),
    };


    format!(
        "{first} hanging on the wall,\n\
         {first} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {second} hanging on the wall.",
    )
}
