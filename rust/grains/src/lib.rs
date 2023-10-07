pub fn square(s: u32) -> u64 {
    match s {
        n if n <= 0 || n > 64 => panic!("Square must be between 1 and 64"),
        n => 2u64.pow(n - 1),
    }
}

pub fn total() -> u64 {
    (1..64).fold(1u64, |acc, step| acc + 2u64.pow(step))
}
