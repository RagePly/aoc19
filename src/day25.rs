// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day25");

#[cfg(not(feature = "day25"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(25, 1)
}

#[cfg(not(feature = "day25"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(25, 2)
}

// v START SOLUTION v

#[cfg(feature = "day25")]
pub fn part1(source: String) -> String {
    String::from("not implemented")
}

#[cfg(feature = "day25")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
