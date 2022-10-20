use std::io;
use std::str;
fn main(){
    let assigment_2: &str = "HelloAssigmentL";
    let mut input = String::new();
    loop {
        println!("Please input 1 character.");
        

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: char = match input.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };
        println!("{}", input);
        // println!("{}", input.len());
        let input_lower = input.to_lowercase().to_string();
        println!("{}", input_lower);
        let input_upper = input.to_uppercase().to_string();
        // let input_lower = input.to_lowercase();
        println!("{}", input_upper);
        let result = str::replace(&assigment_2, &input_upper, "");
        let result_all = str::replace(&result, &input_lower, "");
        // let result = str::replace(&assigment_2, input, "");
        println!("{}", result_all);
        println!("{}", assigment_2.len()-result_all.len());
        break;
    }
    
    // input = 
    // 
    // let mut count: i32 = 0;
    // // for chr in assigment_2.chars(){
    // //     // println!("{}",chr.to_uppercase());
    // //     if chr.to_uppercase().eq(&input.to_uppercase()){
    // //         count = count+1;
    // //     }
    // // }
    
    // let s = assigment_2.replace(&input.to_uppercase(),"");
    
    
    
}