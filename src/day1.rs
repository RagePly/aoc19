// Conditional Compilation
#[cfg(not(feature = "day1"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(1, 1)
}

#[cfg(not(feature = "day1"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(1, 2)
}

// v START SOLUTION v

#[cfg(feature = "day1")]
pub fn part1(source: String) -> i32 {
    source.split("\r\n").into_iter().map::<i32, _>(
        |l| l.parse::<i32>().unwrap() / 3 - 2
    ).sum()
}

fn calc_fuel(m: i32) -> i32 {
    if m == 0 {
        0
    } else {
        let fuel = std::cmp::max(m / 3 - 2, 0);
        fuel + calc_fuel(fuel)
    }
}

#[cfg(feature = "day1")]
pub fn part2(source: String) -> i32 {
    source.split("\r\n").into_iter().map::<i32, _>(
        |l| calc_fuel(l.parse::<i32>().unwrap())
    ).sum()
}
