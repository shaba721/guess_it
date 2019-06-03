

use std::io::stdin;
use rand::Rng;



        pub fn bet_amount() -> u32 {
            println!("Now, how much are we betting?");
            let reader = std::io::stdin();
            loop {
                use std::io::stdin;

                let mut amount = String::new();

                std::io::stdin().read_line(&mut amount)

                    .expect("Failed to read line");
                let amount: u32 = match amount.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => {
                        println!("please enter a number");
                        continue;
                    },
                };
            }
        }



        pub fn range() -> u32 {
            println!("What would you like the max number in the range to be?");
            println!("By default, the lowest range must be 1.");
            let reader = std::io::stdin();
            loop {
                use std::io::stdin;

                let mut custom_range = String::new();

                std::io::stdin().read_line(&mut custom_range)

                    .expect("Failed to read line");
                let amount: u32 = match custom_range.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => {
                        println!("please enter a number");
                        continue;
                    },
                };
            }
        }



        pub fn genned_num(mut x: u32) -> u32 {
            use rand::Rng;

            x += 1;
            let generated_num: u32 = rand::thread_rng().gen_range(1, x);

            return generated_num;
        }


