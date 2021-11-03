use std::time::Instant;

pub mod comp_features;
pub mod intcode;
#[allow(dead_code, unused_variables)] mod day1;
#[allow(dead_code, unused_variables)] mod day2;
#[allow(dead_code, unused_variables)] mod day3;
#[allow(dead_code, unused_variables)] mod day4;
#[allow(dead_code, unused_variables)] mod day5;
#[allow(dead_code, unused_variables)] mod day6;
#[allow(dead_code, unused_variables)] mod day7;
#[allow(dead_code, unused_variables)] mod day8;
#[allow(dead_code, unused_variables)] mod day9;
#[allow(dead_code, unused_variables)] mod day10;
#[allow(dead_code, unused_variables)] mod day11;
#[allow(dead_code, unused_variables)] mod day12;
#[allow(dead_code, unused_variables)] mod day13;
#[allow(dead_code, unused_variables)] mod day14;
#[allow(dead_code, unused_variables)] mod day15;
#[allow(dead_code, unused_variables)] mod day16;
#[allow(dead_code, unused_variables)] mod day17;
#[allow(dead_code, unused_variables)] mod day18;
#[allow(dead_code, unused_variables)] mod day19;
#[allow(dead_code, unused_variables)] mod day20;
#[allow(dead_code, unused_variables)] mod day21;
#[allow(dead_code, unused_variables)] mod day22;
#[allow(dead_code, unused_variables)] mod day23;
#[allow(dead_code, unused_variables)] mod day24;
#[allow(dead_code, unused_variables)] mod day25;

const FORMAT_PART_WIDTH: usize = 13;
const FORMAT_ANSWER_WIDTH: usize = 64;
const FORMAT_TIME_WIDTH: usize = 9;
const FORMAT_TOTAL_WIDTH: usize = 4 + FORMAT_PART_WIDTH + FORMAT_ANSWER_WIDTH + FORMAT_TIME_WIDTH;
const DATA_FOLDER: &'static str = "./data/";

macro_rules! execute_task {
    ($part: path, $source: expr) => {
        let pre = Instant::now();
        let res = $part($source);
        let post = pre.elapsed();
        println!("{0:<3$} {1:<4$} {2:>5$}us", 
            format!("{}:", stringify!($part)),
            res, 
            post.as_micros(),
            FORMAT_PART_WIDTH,
            FORMAT_ANSWER_WIDTH,
            FORMAT_TIME_WIDTH
        );
    };
}

macro_rules! execute_day {
    ($day: path) => {
        {
            use $day as base;
            if cfg!(feature = "use_default") || base::WAS_COMPILED {
                let file_path = format!("{}{}.txt", DATA_FOLDER, stringify!($day));
                match std::fs::read_to_string(file_path.clone())  {
                    Ok(source) => {
                        execute_task!(base::part1, source.clone());
                        execute_task!(base::part2, source);
                    }
                    Err(_) => println!("{0:<2$} puzzle input (file {1}) not found!", 
                        format!("{}:", stringify!($day)), 
                        file_path, 
                        FORMAT_PART_WIDTH)
                }
            }
        }
    };
}

fn main() {
    println!("{0:^1$}", "Advent of Code 2019", FORMAT_TOTAL_WIDTH);
    println!("{0:<3$} {1:<4$} {2:<5$}", 
        "Puzzle:", "Answer", "Time",
        FORMAT_PART_WIDTH, FORMAT_ANSWER_WIDTH, FORMAT_TIME_WIDTH
    );
    println!("{1:-<0$}", FORMAT_TOTAL_WIDTH, "");
    execute_day!(day1);
    execute_day!(day2);
    execute_day!(day3);
    execute_day!(day4);
    execute_day!(day5);
    execute_day!(day6);
    execute_day!(day7);
    execute_day!(day8);
    execute_day!(day9);
    execute_day!(day10);
    execute_day!(day11);
    execute_day!(day12);
    execute_day!(day13);
    execute_day!(day14);
    execute_day!(day15);
    execute_day!(day16);
    execute_day!(day17);
    execute_day!(day18);
    execute_day!(day19);
    execute_day!(day20);
    execute_day!(day21);
    execute_day!(day22);
    execute_day!(day23);
    execute_day!(day24);
    execute_day!(day25);
}
