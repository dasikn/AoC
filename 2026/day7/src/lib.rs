use std::{collections::HashMap, fs::read_to_string};

pub fn beam(i: usize, j: usize, manifold_diagram: &mut Vec<Vec<char>>, counter: &mut i64){
    let no_of_rows = manifold_diagram.len();
    let no_of_columns = manifold_diagram[0].len();

    if i >= no_of_rows {return;}

    if manifold_diagram[i][j] == '|' {return;}

    if manifold_diagram[i][j] == '^'{
        manifold_diagram[i][j] = '*'; // already split
        *counter +=1;
        if j >= 1 {
        beam(i, j-1, manifold_diagram, counter);
        }
        if j+1 < no_of_columns{
            beam(i, j+1, manifold_diagram, counter); 
        }
        return;
    }
    
    if manifold_diagram[i][j] == '.'{
       manifold_diagram[i][j] ='|';
       beam(i+1, j, manifold_diagram, counter);
    }
    
}


pub fn timeline(i: usize, j: usize, manifold_diagram: &Vec<Vec<char>>, memo: &mut HashMap<(usize, usize), i64>) -> i64{
    if i >= manifold_diagram.len() {return 1;}
    if let Some(&cached) = memo.get(&(i, j)) { return cached; } // if already cached return cached timeline count.
    let result = match manifold_diagram[i][j] {
        '.' => timeline(i+1, j, manifold_diagram, memo),
        '^' => timeline(i, j-1, manifold_diagram, memo) + timeline(i, j+1, manifold_diagram, memo),
        _ => 0,
    };
    memo.insert((i, j), result);
    result
}


pub fn solve_day7_2(file_name: &str){
    
    let manifold_diagram = read_file(file_name);
    let mut counter = 0;
    //find start
    for j in 0..manifold_diagram[0].len() {
        if manifold_diagram[0][j] == 'S' {
            let mut memo = HashMap::new();
            counter = timeline(1,j, &manifold_diagram, &mut memo);
            break;
        }
    }    
    println!("The solution is: {:}", counter);
}


pub fn read_file(file_name: &str)-> Vec<Vec<char>>{
    return read_to_string(file_name).unwrap().lines().map(|line| line.chars().collect()).collect();
} 

pub fn print_diagram(manifold_diagram: &Vec<Vec<char>>){
    for i in 0..manifold_diagram.len(){
        for j in 0..manifold_diagram[0].len(){
            print!("{:}", manifold_diagram[i][j]);
        }
        print!("\n");
    }
}

pub fn solve_day7_1(file_name: &str){
    
    let mut manifold_diagram = read_file(file_name);
    let mut counter = 0;
    //find start
    for j in 0..manifold_diagram[0].len() {
        if manifold_diagram[0][j] == 'S' {
            beam(1,j, &mut manifold_diagram, &mut counter);
            break;
        }
    }   
    println!("The solution is: {:}", counter);
}
