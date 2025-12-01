use core::panic;
use std::fs::read_to_string;

pub fn parse_ln(line: &str) -> i32 {
    let sign = if line.starts_with("R") {1} else {-1};
    let distance: i32 = match line[1..].parse(){
        Ok(distance) => distance, 
        Err(error) => panic!("Error w.r.t. distance parsing: {error:?}")
    };
    sign * distance
}

pub fn update_position(position: i32, update: i32) -> (u32, u32) { 
    let number_of_clicks = if update >= 0 {
        (update + position) / 100
    } else {
        ((100 - position) % 100 + update.abs()) / 100
    };
    
    let new_position = ((position + update) % 100 + 100) % 100;
    
    (new_position as u32, number_of_clicks as u32)
}

pub fn solve_day1(file_name: &str, starting_position: u32) -> (i32, i32) {
    let (_, click_counter, zero_counter) = read_to_string(file_name)
        .unwrap()
        .lines()
        .fold((starting_position, 0i32, 0i32), |(pos, clicks, zeros), line| {
            let update = parse_ln(line);
            let (new_pos, new_clicks) = update_position(pos as i32, update);
            (new_pos, clicks + new_clicks as i32, zeros + (new_pos == 0) as i32)
        });
    
    (zero_counter, click_counter)
}