pub fn start_day(day: &str) {
    println!("Advent of Code 2025 - Day {:0>2}", day);
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("day_01");
        start_day("day_02");
        start_day("day_03");
        start_day("day_04");
    }
}
