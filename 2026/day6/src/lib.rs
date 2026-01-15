use std::fs::read_to_string;

    pub fn solve_day6_2(filename: &str){
        let grid: Vec<Vec<char>> = read_to_string(filename).unwrap().lines().map(|line| line.chars().collect()).collect();
        let number_of_rows = grid.len() - 1; 
        let number_of_columns = grid[0].len();
        let mut new_block = true; 
        let mut op : char = ' ';
        let mut block_result = 0; 
        let mut overall_result = 0; 
        for i in 0..number_of_columns{
            if new_block{
                op = grid[number_of_rows][i];
                new_block = false; 
                overall_result += block_result; 
                if op == '*'{
                    block_result = 1; 
                } else{
                    block_result = 0;
                }
            }
            let (number, no_digit_found) = get_number(&grid, i, number_of_rows);
            // new block
            if no_digit_found {
                new_block = true; 
                continue;
            }
            // operation
            if op == '*'{
                block_result *= number; 
            } else{
                block_result += number;
            }
        }
        overall_result += block_result;
        println!("overall result: {:?}", overall_result)

    }


    pub fn get_number(grid: &Vec<Vec<char>>, column: usize, number_of_rows: usize)-> (i64, bool) {

        let mut no_digit_found = true;
        let mut result : i64 = 0;

        for  i in 0..number_of_rows{
            if let Some(number) = grid[i][column].to_digit(10){
                no_digit_found = false; 
                result = (result * 10) + (number as i64);
            }
        }

        (result, no_digit_found)

    }
 
    pub fn solve_day6_1(filename: &str){
        let content = read_to_string(filename).unwrap();
        let line_iter = content.lines(); // a bit unnecessary traverse of the line
        let mut sum = Vec::new();
        let mut prod= Vec::new(); 
        let mut result = 0; 
        let mut first_line = true; 
        for line in line_iter{

            if let Ok(numbers) = line.split_whitespace().map(|x| x.parse::<i64>()).collect::<Result<Vec<i64>, _>>(){
                if first_line{
                    let number_of_columns = numbers.len();
                    sum = vec![0; number_of_columns];
                    prod = vec![1; number_of_columns];
                    first_line = false; 
                }
                update_result(&mut sum, &mut prod, numbers);
            } else{
                let op: Vec<&str> = line.split_whitespace().collect();
                result = complete_results(&sum, &prod, &op)
            } 
            
        }
        println!("{:?}", result);
 

    }

    fn update_result(sum: &mut Vec<i64>, prod: &mut Vec<i64>, numbers: Vec<i64>){
        let no_of_columns = numbers.len();
        for i in 0..no_of_columns{
            sum[i] += numbers[i];
            prod[i] *= numbers[i];
        }   
    }
    
    fn complete_results(sum: &Vec<i64>, prod: &Vec<i64>, op: &Vec<&str>) -> i64 {
        let no_of_columns = sum.len();
        let mut result = 0;
        for i in 0..no_of_columns{
            let value = match op[i] {
                "*" => prod[i],
                "+" => sum[i],
                _ => 0, 
            };
            result += value;
        }
        result
    }
    
   