/* trunk-ignore(clippy/E0554) */
#![feature(iter_next_chunk)]

pub enum Whole128 {
    Signed(i128),
    Unsigned(u128),
}

#[macro_export()]
macro_rules! define_problem {
    ($year:ident::$day:ident::$task:ident, $expected:expr) => {{
        let problem_number = &stringify!($year::$day::$task)[..];
        use $year::$day::$task::ans as ans_func;
        let expected_value = $expected;
        if expected_value < 0 {
            let return_function: &dyn Fn() -> Whole128 = &|| Whole128::Signed(ans_func() as i128);
            (
                return_function,
                Whole128::Signed(expected_value as i128),
                problem_number,
            )
        } else {
            let return_function: &dyn Fn() -> Whole128 = &|| Whole128::Unsigned(ans_func() as u128);
            (
                return_function,
                Whole128::Unsigned(expected_value as u128),
                problem_number,
            )
        }
    }};
}

pub mod year2022;
pub mod year2023;