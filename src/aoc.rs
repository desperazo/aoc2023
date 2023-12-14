pub mod day1;
pub mod day2;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::{day1, day2};

    #[test]
    fn day1_1() {
        let ans = day1::solve();
        assert_eq!(54159, ans);
    }

    #[test]
    fn day1_2() {
        let ans = day1::solve_2();
        assert_eq!(53866, ans);
    }

    #[test]
    fn day2_1() {
        let ans = day2::solve();
        assert_eq!(2265, ans);
    }
}
