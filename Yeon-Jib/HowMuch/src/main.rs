use std::io;
fn input_u32() -> u32{
    let mut input_value = String::new();
    io::stdin().read_line((&mut input_value)).expect("Failed to read line");
    input_value.parse().unwrap()
}
struct Car{
    basic_proce:u32,
    total_option_number:u32,
    option_infos:Vec<(u32,u32)>
}
impl Car {
    fn calc_price(&self)-> u32 {
        let i:u32 =  self.option_infos.iter().map(|x| x.0*x.1).sum();
        self.basic_proce + i
    }
}



fn main() {
    loop{
        let mut car_price = 0;
        let mut option_number = 0 ;
        let mut options: Vec<(u32,u32)> = Vec::new();

        println!("Please input Car Price");
        car_price = input_u32();

        println!("Please input option Number");
        option_number = input_u32();
        
        for index in 0..option_number
        {
            let mut input_str = String::new();

            println!("Please input option");
            io::stdin().read_line((&mut input_str)).expect("Failed to read line");
            let mut option_info:Vec<&str> = input_str.split(" ").collect();
            options.push((option_info[0].trim().parse().unwrap(),option_info[1].trim().parse().unwrap()));
        }
        let car_info: Car = Car{basic_proce:car_price, total_option_number:option_number,option_infos:options};
        print!("{}",car_info.calc_price());
    }
}
