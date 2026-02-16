use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {

        let mut next_guess = (min + max)/2;
        let mut prev_guess = 0;
        let mut binary_boothang= player.ask_to_compare(next_guess);
       
        if binary_boothang == 1 {
            prev_guess = next_guess;
            next_guess = (prev_guess + min)/2; 
        }

        if binary_boothang == -1 {
            prev_guess = next_guess;
            next_guess = (prev_guess + max)/2;
        }
        
        if binary_boothang == 0 {
            return next_guess;
        }

        else {
            return 00000; 
        }
    }
}
