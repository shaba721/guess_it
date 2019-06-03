

use std::io::stdin;
use std::cmp::Ordering;






    pub fn User1_input () -> u32 {
        let mut user1 = String::new();

        loop  {



                println!("user 1 please tell me what your guess for the secrete number");


                std::io::stdin().read_line(&mut user1);

                let user1= match user1.trim().parse() {
                    Ok(num) => num,

                    Err(_) => {
                        println!("please only insert a NUMBER");
                        continue;
                    }
                };

            if user1 < 2000000{
                break user1;
            }

        }
        }



        pub fn User2_input() -> u32 {
            loop {
                println!("User 2 please tell me what your guess is now for the secrete number!");
                let mut user2 = String::new();
                std::io::stdin().read_line(&mut user2);
                let user2: u32 = match user2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("please only insert a NUMBER");
                        continue;
                    },
                };
                if user2 < 2000000{
                    break user2;
                }
            }
        }

