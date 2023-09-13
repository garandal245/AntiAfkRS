extern crate autopilot;
use std::{io, time::Duration};
use rand::Rng;


fn get_user_input(prompt: &str)-> i32 {         // The function requires a prompt when called and assigns it to a string variable

    println!("{}", prompt);                     // Prints the prompt that was given

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


fn compare_cursor_position(timer: &mut i32, max_afk_time: i32, afk: &mut bool){

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
        if *timer >= max_afk_time {
             *afk = true;
             return;
        }

    }
}



fn main() -> ! {
    let mut timer = 0;
    let mut afk = false;



    let max_afk_time = get_user_input("Enter the max afk time desired:");


    println!("please enter the range of coorditantes to click in");
    let  xmin = get_user_input("Enter xmin:");
    let  xmax = get_user_input("Enter xmax:");
    let  ymin = get_user_input("Enter ymin:");
    let  ymax = get_user_input("Enter ymax:");
    println!("Will randomly click between\nX range: {xmin}, {xmax}\nY range: {ymin}, {ymax}");



    loop{
        compare_cursor_position(&mut timer, max_afk_time, &mut afk);

        if !afk {
            continue;
        }
        println!("you were afk for {} seconds, moving mouse", timer);
        timer = 0;
        let xrand = rand::thread_rng().gen_range(xmin..=xmax);
        let yrand = rand::thread_rng().gen_range(ymin..=ymax);
        autopilot::mouse::smooth_move(autopilot::geometry::Point::new(xrand as f64, yrand as f64), Some(100.0));



    }
}