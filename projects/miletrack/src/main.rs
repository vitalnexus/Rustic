use std::io;

fn main() {
    println!("Enter your starting and ending mileage:");

    loop {
        println!("Enter starting miles:");
        let mut mileage_s = String::new();
    
        io::stdin() 
            .read_line(&mut mileage_s)
            .expect("Cannot accept mileage!");

        let mileage_s: u32 = match mileage_s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };    
        println!("Start miles is: {mileage_s}");

        println!("Enter ending miles:");
        let mut mileage_e = String::new();
       
        io::stdin()
           .read_line(&mut mileage_e)
           .expect("Cannot accept mileage!");
    
        let mileage_e: u32 = match mileage_e.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("End miles is: {mileage_e}");
   
        let total_miles: u32 = {mileage_e} - {mileage_s};
 
        println!("Mileage driven is: {total_miles}");
        break;
    }
}
