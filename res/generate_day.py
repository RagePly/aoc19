import os
from sys import argv

SOURCE_DIR = "src"
CONTENT = """// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day#DAY#");

#[cfg(not(feature = "day#DAY#"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(#DAY#, 1)
}

#[cfg(not(feature = "day#DAY#"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(#DAY#, 2)
}

// v START SOLUTION v

#[cfg(feature = "day#DAY#")]
pub fn part1(source: String) -> String {
    String::from("not implemented")
}

#[cfg(feature = "day#DAY#")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
"""

print_use = lambda: print(f"use: py {__file__} N, where N is the day")

if __name__ == "__main__":

    if not len(argv) == 2:
        print_use()
        quit()

    try:
        day_nr = int(argv[1])
    except:
        print(f"invalid argument {argv[1]}")
        print_use()
        quit()

    file_name = f"day{day_nr}.rs"
    file_path = SOURCE_DIR + "/" + file_name

    if file_name in os.listdir(SOURCE_DIR):
        print(f"{file_path} already exists.")
        quit()

    with open(file_path, "w") as f:
        f.write(CONTENT.replace("#DAY#", str(day_nr)))

    print(f"Don't forget to add \'mod day{day_nr};\' as well as calling the method(s) in src/main.rs")
else:
    f"{__file__} cannot be run as library"

quit()