pub mod day1;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::day1;

    #[test]
    fn day1_1() {
        let ans = day1::solve();
        assert_eq!(54159, ans);
    }
}
