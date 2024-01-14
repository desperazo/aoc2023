pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::{day1, day10, day11, day2, day3, day4, day5, day6, day7, day8, day9};

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

    #[test]
    fn day5_2() {
        let ans = day5::solve_2();
        assert_eq!(1928058, ans);
    }

    #[test]
    fn day6() {
        let ans = day6::solve();
        assert_eq!(505494, ans);
    }

    #[test]
    fn day6_2() {
        let ans = day6::solve_2();
        assert_eq!(23632299, ans);
    }

    #[test]
    fn day7() {
        let ans = day7::solve();
        assert_eq!(248396258, ans);
    }

    #[test]
    fn day7_2() {
        let ans = day7::solve_2();
        assert_eq!(246436046, ans);
    }

    #[test]
    fn day8() {
        let ans = day8::solve();
        assert_eq!(20659, ans);
    }

    #[test]
    fn day8_2() {
        let ans = day8::solve_2();
        assert_eq!(15690466351717, ans);
    }

    #[test]
    fn day9() {
        let ans = day9::solve();
        assert_eq!(2175229206, ans);
    }

    #[test]
    fn day9_2() {
        let ans = day9::solve_2();
        assert_eq!(942, ans);
    }

    #[test]
    fn day10() {
        let ans = day10::solve();
        assert_eq!(7030, ans);
    }

    #[test]
    fn day10_2() {
        let ans = day10::solve_2();
        assert_eq!(285, ans);
    }

    #[test]
    fn day11() {
        let ans = day11::solve();
        assert_eq!(9686930, ans);
    }

    #[test]
    fn day11_2() {
        let ans = day11::solve_2();
        assert_eq!(630728425490, ans);
    }
}
