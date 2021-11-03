// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day2");

#[cfg(not(feature = "day2"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(2, 1)
}

#[cfg(not(feature = "day2"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(2, 2)
}

// v START SOLUTION v
#[cfg(feature = "day2")] use super::intcode;


#[cfg(feature = "day2")]
pub fn part1(source: String) -> intcode::Int {
    let mut vm = intcode::IntcodeState::new(source.as_str()).unwrap(); 
    vm.memory[1] = 12;
    vm.memory[2] = 2;
    vm.run();
    vm.memory[0]
}

#[cfg(feature = "day2")]
pub fn part2(source: String) -> intcode::Int {
    let mut vm = intcode::IntcodeState::new(source.as_str()).unwrap(); 
    for noun in 0..100 {
        for verb in 0..100 {
            vm.memory[1] = noun;
            vm.memory[2] = verb;
            vm.run();
            if vm.memory[0] == 19690720 {
                return 100*noun + verb;
            }
            vm.reset();
        }
    }
    panic!("no solution found")
}
