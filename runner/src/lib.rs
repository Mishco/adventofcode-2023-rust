// source: https://github.com/timvisee/advent-of-code-2023/tree/master/runner

pub fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (day01a::main, "day01a"),
        (day01b::main, "day01b"),
    ]
}