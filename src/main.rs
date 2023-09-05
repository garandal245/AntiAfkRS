extern crate autopilot;
use std::{io, time::Duration};


fn get_user_time()-> i32 {

    loop {          // Loop until a valid number is provided.

        let mut input = String::new();  // Creates a mutable variable "input" as a string
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");     // Calls io::stdin.readline and assigns it by using a mutable reference to input
                                                // Reference is essentially a pointer with a guarenteed lifetime and ownership concept
                                                // See reference test in src\learn\references



        match input.trim().parse::<i32>() {     // Parses the variable "input(string)" to an i32
            Ok(number) => {                     // If ok it assigns it to a newly created variable "number"
                return number;                  // Returns number to the calling function
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;                       // Continue the loop if parsing fails
            }
        }
    };

}


fn compare_cursor_position(timer: &mut i32, usertime: i32, afk: &mut bool){

    loop {
        let cursor_locationold = autopilot::mouse::location();
        std::thread::sleep(Duration::from_secs(1));
        let cursor_locationnew = autopilot::mouse::location();
        if cursor_locationold == cursor_locationnew {
            *timer += 1;
            println!("{}", timer);
        } else {
            *timer = 0;
            println!("{}", timer);
        }
        if *timer >= usertime {
            println!("You are AFK");
             *afk = true;
             return;
        }

    }
}

fn main() {
    let mut timer = 0;
    let mut afk = false;
    println!("Enter max desired AFK time (in seconds)");


    let usertime = get_user_time();

    loop{
        compare_cursor_position(&mut timer, usertime, &mut afk);



        if afk {
            println!("you were afk for {} seconds", timer);
            timer = 0;
        }

    }
}