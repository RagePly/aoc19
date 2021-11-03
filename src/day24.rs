// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day24");

#[cfg(not(feature = "day24"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(24, 1)
}

#[cfg(not(feature = "day24"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(24, 2)
}

// v START SOLUTION v

#[cfg(feature = "day24")]
pub fn part1(source: String) -> String {
    String::from("not implemented")
}

#[cfg(feature = "day24")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
