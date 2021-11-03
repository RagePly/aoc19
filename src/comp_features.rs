#[allow(dead_code)]
pub fn fetch_default(day: usize, part: usize) -> String {
    if 2 < part {
        format!("Invalid part {}", part)
    } else {
        let p = format!("res/default/day{}.txt", day);

        match std::fs::read_to_string(p) {
            Ok(p) => match p.splitn(2, "\r\n").into_iter().skip(part - 1).next() {
                Some(r) => String::from(r),
                None => String::from("couldn't interpret file")
            }
            Err(_) => String::from("not completed")
        }
    }
}