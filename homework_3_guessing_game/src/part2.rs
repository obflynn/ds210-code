use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut low_val = min;
        let mut high_val = max;

        while low_val < high_val { // while loop suggested by darling ChatGPT since the program would only go through one iteration w/o it and I counldn't figure out why
            let next_guess: u32 = (low_val + high_val)/2;
            let binary_boothang = player.ask_to_compare(next_guess);
            
            if binary_boothang == 0 { 
                return next_guess; // user entered "e"
            }

            else if binary_boothang == -1 { // originally had this as 1, but for reasons that perplex me that was incorrect and caused the next guess a number greater than the previous one
                high_val = next_guess - 1; // user entered "l"
            }

            else if binary_boothang == 1 { // originally had this as -1, but for reasons that perplex me that was incorrect and caused the next guess a number less than the previous one
                low_val = next_guess + 1; // user entered "g"
            }
        }

        return 00000; // return value added to satisfy the return type since program will throw an error w/o it
    }
}
