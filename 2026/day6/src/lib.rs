use std::fs::read_to_string;
 
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
    
   