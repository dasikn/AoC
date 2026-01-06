use std::fs::read_to_string;

fn count_successes(row: usize, roll_grid: &mut Vec<Vec<u8>>, c: usize, r: usize, symbol:u8) -> i32 {
    
    let mut success_counter = 0;
    
    for col in 0..c{
        if roll_grid[row][col] == symbol{
            let mut counter = 0;
            let start_col = if col > 0 { col - 1 } else { 0 };
            let end_col = (col + 2).min(c);
            for neighbor_col in start_col..end_col{
                if counter < 4 {
                    let start_row = if row > 0 { row - 1 } else { 0 };
                    let end_row = (row + 2).min(r);
                    for neighbor_row in start_row..end_row{
                        if (row == neighbor_row) & (col == neighbor_col){continue;}
                        if counter < 4 {
                            if roll_grid[neighbor_row][neighbor_col] == symbol {
                                counter += 1;
                            }
                        }
                    }
                }
            }
            if counter < 4 {
                success_counter += 1;
                roll_grid[row][col] = ".".as_bytes()[0];
            }
        }
    }

    success_counter

}

fn count_successes_alt(row: usize, roll_grid: &mut Vec<Vec<u8>>, c: usize, r: usize, symbol:u8, symbol_to_be_released: u8) -> i32 {
    
    let mut success_counter = 0;
    
    for col in 0..c{
        if roll_grid[row][col] == symbol{
            let mut counter = 0;
            let start_col = if col > 0 { col - 1 } else { 0 };
            let end_col = (col + 2).min(c);
            for neighbor_col in start_col..end_col{
                if counter < 4 {
                    let start_row = if row > 0 { row - 1 } else { 0 };
                    let end_row = (row + 2).min(r);
                    for neighbor_row in start_row..end_row{
                        if (row == neighbor_row) & (col == neighbor_col){continue;}
                        if counter < 4 {
                            if (roll_grid[neighbor_row][neighbor_col] == symbol) || (roll_grid[neighbor_row][neighbor_col] == symbol_to_be_released) {
                                counter += 1;
                            }
                        }
                    }
                }
            }
            if counter < 4 {
                success_counter += 1;
                roll_grid[row][col] = symbol_to_be_released;
            }
        }
    }

    success_counter

}

fn clear_row(roll_grid: &mut Vec<u8>, symbol_to_be_released: u8, symbol_gone: u8) {
    for c in roll_grid.iter_mut() {
        if *c == symbol_to_be_released {
            *c = symbol_gone; 
        }
    }
}

pub fn count_successes_new(file_name: &str) -> i32 {
    let mut count_success; 
    let mut count_success_new = 0; 

    let symbol = "@".as_bytes()[0];
    let symbol_to_be_released = "X".as_bytes()[0];
    let symbol_gone = ".".as_bytes()[0];

    let content = read_to_string(file_name).unwrap();
    let mut roll_grid: Vec<Vec<u8>> = content
      .lines()
      .map(|line| line.as_bytes().to_vec())
      .collect();

    let r = roll_grid.len();  // number of rows
    let c = roll_grid[0].len();  // number of columns (assumes non-empty)

    loop{
           count_success = count_success_new; 
           let mut success_counter = 0; 
           for row in 0..r {
                success_counter += count_successes_alt(row, &mut roll_grid, c,r, symbol, symbol_to_be_released);
                if row > 0{
                     clear_row(&mut roll_grid[row-1], symbol_to_be_released, symbol_gone);
                 }
             }
            clear_row(&mut roll_grid[r-1], symbol_to_be_released, symbol_gone);
        
        count_success_new += success_counter;

        if count_success == count_success_new{
            break; 
        }
    }

    return count_success_new;
    
}



pub fn solve_day4_alt(file_name: &str) -> i32{

    count_successes_new(file_name)

}

pub fn solve_day4(file_name: &str) -> i32{

    let symbol = "@".as_bytes()[0];

    let mut roll_grid: Vec<Vec<u8>> = Vec::new();

    let content = read_to_string(file_name).unwrap();
    let mut line_iter = content.lines();

    // you always need one line in the future
    let line = line_iter.next().unwrap();
    roll_grid.push(line.as_bytes().to_vec());
    let c = line.as_bytes().len(); 
    let mut success_counter = 0; 

    let mut row = 0; 
    let mut r = 1; 

    for line in line_iter{
        roll_grid.push(line.as_bytes().to_vec());
        r += 1; 
        success_counter += count_successes(row, &mut roll_grid, c,r, symbol);
        row += 1;
    }

    // last line we've already read in.
    success_counter += count_successes(row, &mut roll_grid, c,r, symbol);

    success_counter

}