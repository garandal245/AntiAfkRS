        extern crate autopilot;
        use std::{io, time::Duration};

        fn main() {
            let mut timer = 0;

            println!("Enter max desired AFK time (in seconds)");

            let mut usertime = loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        break number; // Exit the loop when a valid number is provided
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue; // Continue the loop if parsing fails
                    }
                }
            };


            loop {
                let cursor_locationold = autopilot::mouse::location();
                std::thread::sleep(Duration::from_secs(1));
                let cursor_locationnew = autopilot::mouse::location();

                if cursor_locationold == cursor_locationnew {
                    timer += 1;
                    println!("{}", timer);
                } else {
                    timer = 0;
                    println!("{}", timer);
                }

                if timer >= usertime {
                    println!("You are AFK");
                    break;
                }
            }
        }
