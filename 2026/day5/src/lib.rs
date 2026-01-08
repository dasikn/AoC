use std::{fs:: read_to_string};
use std::str::Lines;


pub fn solve_day5(filename: &str) {
    // returns how many of the ingredients are fresh. 
    let content = read_to_string(filename).unwrap();
    let mut line_iter = content.lines();
    let range_vec = create_ranges(&mut line_iter);
    let counter = check_ids(&mut line_iter, &range_vec);
    println!("{:?}",counter);
}

pub fn solve_day5_2(filename: &str){
    let content = read_to_string(filename).unwrap();
    let mut line_iter = content.lines();
    let mut range_vec = create_ranges(&mut line_iter);
    let counter = merge_all(&mut range_vec);
    println!("solution to solve_day5_2: {:?}", counter);
}

pub fn merge_all(range_vec: &mut Vec<[i64;2]>) -> i64{
    let mut idx = 0; 
    let mut counter = 0; 

    let mut current_length = range_vec.len();

    while idx < current_length -1 {
        let range_a = range_vec[idx];
        let range_b = range_vec[idx+1];
        if let Some(merged_array) = merge_ranges(range_a, range_b){
            range_vec.remove(idx+1);
            current_length -= 1;
            range_vec[idx] = merged_array;
        }else {
            counter += (range_vec[idx][1] - range_vec[idx][0]) + 1;
            idx +=1;  
        }
    }
    if idx == (current_length - 1){
        counter += (range_vec[idx][1] - range_vec[idx][0]) + 1;
    }
    counter
}


fn merge_ranges(range_a: [i64; 2], range_b: [i64; 2]) -> Option<[i64; 2]>{
    // assumption: range_a[0] < range_b[0], we will sort the ranges like this
    let mut new_range = [0; 2];
    if range_b[0] <= range_a[1] + 1{
        new_range[0] = range_a[0];
        if range_b[1] > range_a[1]{
            new_range[1] = range_b[1];
            return Some(new_range);
        }
        new_range[1] = range_a[1];
        return Some(new_range);
    } 
    return None;
}

fn create_ranges(line_iter: &mut Lines<'_>) -> Vec<[i64; 2]>{

    // create a vector on the heap that holds the reanges sorted by minimum of the range
    let mut range_vec = Vec::<[i64;2]>::new();
    
    for line in line_iter{
        if line.is_empty(){
            break; 
        }
        let el = parse_line_to_range(line);
        sort_element_into_vec(el, &mut range_vec);
    }
    return range_vec; 
}

fn parse_line_to_range(line: &str)-> [i64; 2]{
    let nums: [i64; 2] = line.split_once('-')
    .map(|(a, b)| [a.parse().unwrap(), b.parse().unwrap()])
    .unwrap();
    nums
}

fn check_ids(line_iter: &mut Lines<'_>, range_vec: &Vec<[i64;2]>) -> i64 {
    let mut counter = 0; 
    for line in line_iter{
        let id = parse_line_to_id(line);
        if is_fresh(id, range_vec){counter += 1};
    }
    counter
}

fn is_fresh(id: i64, range_vec: &Vec<[i64;2]>) -> bool {
    for el in range_vec{
        if id < el[0]{
            return false;
        }
        if id <= el[1]{
            return true; 
        }
    }
    false
}

fn parse_line_to_id(line: &str) -> i64 {
    line.parse().unwrap()
}


// sort a tuple into a already sorted vector
fn sort_element_into_vec(el: [i64; 2], range_vec: &mut Vec<[i64; 2]>){
    // else sort into sorted vector 
    for idx in 0..range_vec.len(){
        if el[0] <= range_vec[idx][0]{
            range_vec.insert(idx, el);
            return;
        }
    }
    range_vec.push(el);
}