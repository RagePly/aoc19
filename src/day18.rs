// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day18");

#[cfg(not(feature = "day18"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(18, 1)
}

#[cfg(not(feature = "day18"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(18, 2)
}

// v START SOLUTION v

#[cfg(feature = "day18")]
pub fn part1(source: String) -> String {
    String::from("not implemented")
}

#[cfg(feature = "day18")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
