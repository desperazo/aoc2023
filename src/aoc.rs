pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::{day1, day2, day3, day4, day5};

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

    #[test]
    fn day2_2() {
        let ans = day2::solve_2();
        assert_eq!(64097, ans);
    }

    #[test]
    fn day3_1() {
        let ans = day3::solve();
        assert_eq!(531561, ans);
    }

    #[test]
    fn day3_2() {
        let ans = day3::solve_2();
        assert_eq!(83279367, ans);
    }

    #[test]
    fn day4_1() {
        let ans = day4::solve();
        assert_eq!(20855, ans);
    }

    #[test]
    fn day4_2() {
        let ans = day4::solve_2();
        assert_eq!(5489600, ans);
    }

    #[test]
    fn day5_1() {
        let ans = day5::solve();
        assert_eq!(20855, ans);
    }
}
