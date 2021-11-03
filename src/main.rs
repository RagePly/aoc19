use std::time::Instant;

pub mod comp_features;
#[allow(dead_code)] mod day1;

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
        let file_path = format!("{}{}.txt", DATA_FOLDER, stringify!($day));
        match std::fs::read_to_string(file_path.clone())  {
            Ok(source) => {
                use $day as base;
                execute_task!(base::part1, source.clone());
                execute_task!(base::part2, source);
            }
            Err(_) => println!("{0:<2$} puzzle input (file {1}) not found!", 
                format!("{}:", stringify!($day)), 
                file_path, 
                FORMAT_PART_WIDTH)
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
}
