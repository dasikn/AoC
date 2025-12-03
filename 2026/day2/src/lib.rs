use std::fs; 

pub fn is_valid(number: &str) -> bool {
    let byte_array = number.as_bytes(); // this is cheap as the referent is not transmuted or copied
    return !is_repeated(byte_array);
}


pub fn is_repeated(byte_array: &[u8]) -> bool {
    let byte_size = byte_array.len(); 
    if byte_size % 2 == 1 {return false}
    let mut first_half_idx = 0; 
    let mut second_half_idx = byte_size / 2; 
    while  first_half_idx < byte_size / 2 {
        if byte_array[first_half_idx] != byte_array[second_half_idx] {
            return false
        }
        first_half_idx +=1; 
        second_half_idx+=1; 

    }
    true
}

pub fn is_repeated_beginning(byte_array: &[u8]) -> bool {

    let byte_size = byte_array.len();
    let mut to_idx = 2; 
    while to_idx < byte_size {
        if is_repeated(&byte_array[0..to_idx]){return true}
        to_idx +=2;
    }
    false
}

pub fn is_repeatition_of_size(byte_array: &[u8], size: usize)-> bool {
    // if it is not a multiple it cannot
    if byte_array.len() % size != 0 {return false};
    let a_slice = &byte_array[0..size];
    let mut from_idx = size; 
    let mut to_idx = from_idx + size; 
    while to_idx <= byte_array.len() {
        let comp_slice = &byte_array[from_idx..to_idx];
        if a_slice == comp_slice {
            from_idx += size; 
            to_idx += size; 
        } else {
            return false
        }
    }
    true

}

pub fn is_valid_3(number: &str) -> bool {
    let byte_array = number.as_bytes(); 
    let size_of_array = byte_array.len();
    let mut size_counter = 1;
    let mut size_is_odd = true;
    if size_of_array % 2 == 0{
        size_is_odd = false;
    }
    while size_counter <= size_of_array / 2 {
        if is_repeatition_of_size(byte_array, size_counter){
            return false;
        }
        if size_is_odd{
            size_counter += 2;
        } else {
            size_counter += 1;
        }

    }
    true
}


// I first understood it like this, ANY repetiton
pub fn is_valid_2(number: &str) -> bool { 
    let byte_array = number.as_bytes();
    let byte_size = byte_array.len();
    for i in 0..byte_size{
        if is_repeated(&byte_array[i..byte_size]){
            return false;
        }
    }
    true
}

pub fn solve_day2_2(file_name: &str) -> i64 {
    let mut solution = 0;
    let file = fs::read_to_string(file_name).expect("something wrong");
    for range in file.split(","){
        let mut min_max = range.split("-");
        let min: i64 = min_max.next().unwrap().parse().expect("parsing failed");
        let max: i64 = min_max.next().unwrap().parse().expect("parsing failed");
        for i in min..(max+1){
            if !is_valid_3(i.to_string().as_str()){
                println!("invalid: {}", i);
                solution+=i;
            }
        }
        
    }   
    solution
}

pub fn solve_day2(file_name: &str) -> i64 {
    let mut solution = 0;
    let file = fs::read_to_string(file_name).expect("something wrong");
    for range in file.split(","){
        let mut min_max = range.split("-");
        let min: i64 = min_max.next().unwrap().parse().expect("parsing failed");
        let max: i64 = min_max.next().unwrap().parse().expect("parsing failed");
        for i in min..(max+1){
            if !is_valid(i.to_string().as_str()){
                println!("invalid: {}", i);
                solution+=i;
            }
        }
        
    }   
    solution
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_is_valid(){
        assert_eq!(is_valid("1234"), true);
    }
    #[test]
    fn test_bad_is_valid(){
        assert_eq!(is_valid("1212"), false);
    }



}