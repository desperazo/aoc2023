pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
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
    use crate::aoc::{
        day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20,
        day21, day22, day23, day3, day4, day5, day6, day7, day8, day9,
    };

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

    #[test]
    fn day12() {
        let ans = day12::solve();
        assert_eq!(7922, ans);
    }

    #[test]
    fn day12_2() {
        let ans = day12::solve_2();
        assert_eq!(18093821750095, ans);
    }

    #[test]
    fn day13() {
        let ans = day13::solve(false);
        assert_eq!(37975, ans);
    }

    #[test]
    fn day13_2() {
        let ans = day13::solve(true);
        assert_eq!(32497, ans);
    }

    #[test]
    fn day14() {
        let ans = day14::solve();
        assert_eq!(109654, ans);
    }

    #[test]
    fn day14_2() {
        let ans = day14::solve_2();
        assert_eq!(94876, ans);
    }

    #[test]
    fn day15() {
        let ans = day15::solve();
        assert_eq!(519041, ans);
    }

    #[test]
    fn day15_2() {
        let ans = day15::solve_2();
        assert_eq!(260530, ans);
    }

    #[test]
    fn day16() {
        let ans = day16::solve();
        assert_eq!(6816, ans);
    }

    #[test]
    fn day16_2() {
        let ans = day16::solve_2();
        assert_eq!(8163, ans);
    }

    #[test]
    fn day17() {
        let ans = day17::solve();
        assert_eq!(1023, ans);
    }

    #[test]
    fn day17_2() {
        let ans = day17::solve_2();
        assert_eq!(1165, ans);
    }

    #[test]
    fn day18() {
        let ans = day18::solve();
        assert_eq!(35244, ans);
    }

    #[test]
    fn day18_2() {
        let ans = day18::solve_2();
        assert_eq!(85070763635666, ans);
    }

    #[test]
    fn day19() {
        let ans = day19::solve();
        assert_eq!(350678, ans);
    }

    #[test]
    fn day19_2() {
        let ans = day19::solve_2();
        assert_eq!(124831893423809, ans);
    }

    #[test]
    fn day20() {
        let ans = day20::solve();
        assert_eq!(666795063, ans);
    }

    #[test]
    fn day20_2() {
        let ans = day20::solve_2();
        assert_eq!(253302889093151, ans);
    }

    #[test]
    fn day21() {
        let ans = day21::solve();
        assert_eq!(3751, ans);
    }

    #[test]
    fn day21_2() {
        let ans = day21::solve_2();
        assert_eq!(619407349431167, ans);
    }

    #[test]
    fn day22() {
        let ans = day22::solve();
        assert_eq!(530, ans);
    }

    #[test]
    fn day22_2() {
        let ans = day22::solve_2();
        assert_eq!(93292, ans);
    }

    #[test]
    fn day23() {
        let ans = day23::solve();
        assert_eq!(94, ans);
    }
}
