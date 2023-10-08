pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(acc, idx), value| {
            value
                .to_digit(10)
                .map(|digit| {
                    if idx % 2 == 1 {
                        2 * digit % 10 + digit / 5
                    } else {
                        digit
                    }
                })
                .map(|digit| (acc + digit, idx + 1))
        })
        .map_or(false, |(acc, idx)| idx > 1 && acc % 10 == 0)
}
