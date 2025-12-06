use core::num;
use std::fs::read_to_string;


pub fn read_in_array(joltage_bank: &str) -> &[u8]{
joltage_bank.as_bytes()
}

// one-pass solution
pub fn get_joltage(joltage_array: &[u8])-> i32 { 
    let mut value_0 = joltage_array[0]; 
    let mut value_1 = joltage_array[1]; 
    let mut idx = 1; 
    while idx < joltage_array.len() {
        if (value_0 < joltage_array[idx]) & (idx < joltage_array.len() -1) {
            value_0 = joltage_array[idx];
            value_1 = joltage_array[idx+1];
        } else {
            if value_1 < joltage_array[idx] {
                value_1 = joltage_array[idx];
            }
        }
        idx +=1;
    }
    (10 * (value_0 - 48) + (value_1 -48)) as i32
}


//one-pass solution
pub fn get_joltage_2(joltage_array: &[u8], number_of_digits: usize)-> i64 { 

    let mut joltage_idx: Vec<u8> = (0..number_of_digits as u8).collect();
    
    let mut array_idx = 1;

    while array_idx < joltage_array.len(){
        // check the first digit that is smaller than the current number
        for idx in 0..number_of_digits{
            // if already used break
            if array_idx == joltage_idx[idx] as usize {
                break;
            }
            if joltage_array[joltage_idx[idx] as usize] < joltage_array[array_idx]{
                // check if it fits
                if array_idx + number_of_digits - idx <= joltage_array.len(){
                    // update indexes
                    for i in 0..(number_of_digits-idx){
                        joltage_idx[idx + i] = (array_idx + i) as u8; 
                    }
                    break;   
                }
            }
        }
        array_idx +=1;
    }
    let mut number: i64 = 0; 
    let mut base: i64 = 1; 

    for i in (0..number_of_digits).rev() {
        number += base*((joltage_array[joltage_idx[i] as usize] - 48) as i64);
        println!("{:?}", number);
        base = 10*base; 

    }
    number
}
    



pub fn solve_day3_1(file_name: &str) -> i32 {

    let mut counter = 0; 

    for line in read_to_string(file_name). unwrap().lines(){
        counter += get_joltage(read_in_array(line));
    }

    counter

}

pub fn solve_day3_2(file_name: &str, number_of_digits: usize) -> i64 {

    let mut counter = 0; 

    for line in read_to_string(file_name). unwrap().lines(){
        counter += get_joltage_2(read_in_array(line), number_of_digits);
    }

    counter

}

