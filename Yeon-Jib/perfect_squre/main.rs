use std::io;

struct Square{
    min: i32,
    max: i32,
    limit:i32
}
impl Square{
    fn perfect_squre(&self) -> Vec<i32>
    {
        let min = self.min;
        let max = self.max;
        let limit = self.limit;
        let mut prefect_values: Vec<i32> = Vec::new();
        if limit < max || limit < min
        {
            panic!("finded limit number");
        }
        if max !=0 && min < max {
            for number in 1..(max+1) {
                if (number * number) <= max {
                    if min<(number * number)
                    {
                        prefect_values.push(number * number);
                        println!("{}", number);
                    }
                }
                else
                {
                    break;
                }
            }
        }
        prefect_values
    }
}


fn sum_vector(vec: &Vec<i32>)->i32{
    let mut result:i32 = 0;
    for number in vec{
        result+=number;
        println!("{}", number);
    }
    result
}

fn main() {
    let limit = 10001;
    loop 
    {
        let mut min = String::new();
        let mut max = String::new();

        println!("please input min num");
        io::stdin().read_line(&mut min).expect("Failed to min read line");
        let min: i32 = min.trim().parse().expect("Please type a number!");
        println!("please input max num");
        io::stdin().read_line(&mut max).expect("Failed to max read line");
        let max: i32 = max.trim().parse().expect("Please type a number!");
        let squre = Square{min, max, limit}; 
        let prefect_values: Vec<i32> = squre.perfect_squre();
        println!("sum: {}",sum_vector(&prefect_values));
        println!("Min: {}",prefect_values[0]);
    }
}



