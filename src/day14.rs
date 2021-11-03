// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day14");

#[cfg(not(feature = "day14"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(14, 1)
}

#[cfg(not(feature = "day14"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(14, 2)
}

// v START SOLUTION v

#[cfg(feature = "day14")]
pub fn part1(source: String) -> String {
    String::from("not implemented")
}

#[cfg(feature = "day14")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
