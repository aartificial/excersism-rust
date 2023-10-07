pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    '*' => '*',
                    _ => match minefield[i.saturating_sub(1)..=(i + 1).min(minefield.len() - 1)]
                        .iter()
                        .fold(0, |acc, slice| {
                            slice[j.saturating_sub(1)..=(j + 1).min(minefield[0].len() - 1)]
                                .as_bytes()
                                .iter()
                                .fold(acc, |acc, ch| match ch {
                                    b'*' => acc + 1,
                                    _ => acc,
                                })
                        }) {
                        0 => ' ',
                        n if n > 0u32 => std::char::from_digit(n, 10).unwrap(),
                        n => panic!("Invalid number of mines: {}", n),
                    }
                })
                .collect()
        })
        .collect()
}