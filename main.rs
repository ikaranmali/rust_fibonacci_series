use std::io;


fn main() {

    loop{
        println!("\n Fibonacci Series in Rust Prog Lan..");
        println!("\n Press 99 to exit!!");
        println!("\n Enter the nth number for Fibonacci_series:");

        let mut nth_number = String::new();

        io::stdin().read_line( &mut nth_number)
            .expect("Failed to read l");
    
        let nth_number :u32 = match nth_number.trim().parse(){
            Ok(num) =>num,
            Err(_)=>continue,
    };

        // println!("\n number {}",nth_number);
        
        if nth_number == 99{
            println!("\n Exit");
            break
        }

        // let mut temp:u32 = 0;
        let mut a = 0;
        let mut b = 1;
        for _number in 0..nth_number{    
            let  c = a+b;
            a = b;
            b = c;
            println!("\n{}",c);
        }
    }    

}
