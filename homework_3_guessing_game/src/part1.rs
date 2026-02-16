use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut next_guess = min;
        while next_guess < max {
            if player.ask_if_equal(next_guess) {
                return next_guess;
            }
            next_guess += 1; 
        }
        return 00000; //  non-sensical return value added to satisfy the return type in case the user does not answer "y" to any number
    }
}
