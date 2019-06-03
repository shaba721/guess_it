
mod set_up;
mod genning_rangenum;
mod user_guesses;
use rand::Rng;
use std::{io};
use std::io::{Stdin, BufReader};
use crate::set_up::bet_amount;
use crate::set_up::range;
use crate::genning_rangenum::gen_numbs;
use crate::user_guesses::User1_input;
use crate::user_guesses::User2_input;
use crate::set_up::genned_num;



fn main() {



    println!("Welcome to the gambling bot\n");
    println!("Lets play odds. \n");

    let mut Realrange = range();
    let generator=  genned_num(Realrange);
    let Bet_amount=  bet_amount();

println!("The current bet is ${} and the range is 1-{} \n", Bet_amount, Realrange);

       let user1_guess =  user_guesses::User1_input();

       let user2_guess =user_guesses::User2_input();




    if user1_guess == generator {
        println!("user 1 has won ${}", Bet_amount);
        return;
    }
    else if user2_guess == generator {
        println!("user 2 has won ${}", Bet_amount);
        return;

    }
    else {
        println!("Both users has guessed the wrong number so neither user 1 or 2 has won {}", Bet_amount);
        println!("the random number between 1 and {} was {}", Realrange, generator );
        return;
    }

}
