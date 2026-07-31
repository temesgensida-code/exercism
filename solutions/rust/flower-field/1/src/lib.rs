pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return Vec::new();
    }
    let width = garden[0].len();
    let grid: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();
    (0..height)
        .map(|r| {
            (0..width)
                .map(|c| {
                    // If the current square is a flower, keep it as a flower
                    if grid[r][c] == b'*' {
                        '*'
                    } else {
                        // Count adjacent flowers across all 8 possible directions
                        let flower_count = (r.saturating_sub(1)..=std::cmp::min(r + 1, height - 1))
                            .flat_map(|nr| {
                                (c.saturating_sub(1)..=std::cmp::min(c + 1, width - 1))
                                    .map(move |nc| (nr, nc))
                            })
                            .filter(|&(nr, nc)| grid[nr][nc] == b'*')
                            .count();
                        match flower_count {
                            0 => ' ',
                            _ => std::char::from_digit(flower_count as u32, 10).unwrap(),
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}
