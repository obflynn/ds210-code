use crate::player::{self, PlayerTrait};

pub struct SimulatedPlayer {
    the_number: u32,
}
impl SimulatedPlayer {
    pub fn new(number: u32) -> SimulatedPlayer {
        SimulatedPlayer {
            the_number: number
        }
    }
}
impl PlayerTrait for SimulatedPlayer {
    /**
     * This function compares the guess to the number that the player has in mind.
     * The function should return true if the guess and number are equal.
     * The function should return false otherwise.
     */
    fn ask_if_equal(&mut self, guess: u32) -> bool {
        
        if guess == self.the_number {
            return true; // because dur
        }
        else {
            return false;
        }
    }
    /**
     * This function compares the guess to the number that the player has in mind.
     * The function should return one of these three cases:
     * 0 if the number == guess
     * -1 if the number < guess
     * 1 if the number > guess
     */
    fn ask_to_compare(&mut self, guess: u32) -> i32 {
       
        if guess == self.the_number {
            return 0;
        }
        if guess > self.the_number {
            return -1;
        }
        else { // aka if guess < self.the_number
            return 1;
        }
    }
}



#[cfg(test)]
mod part1_tests {
    use crate::part1::Part1;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert_eq!(player.steps(), 1);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 50;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }
}




#[cfg(test)]
mod bad_strategy_tests {
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::{BadStrategy, Strategy};

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }

    #[test]
    #[should_panic]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 7; // not sure if this is what you meant implementing "missing logic", but fear not the test still fails

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }
}


#[cfg(test)]
mod part2_tests {
    use crate::part2::Part2;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    // Add tests for part2 similar to part1 above
    // Note that your tests should test part2, not part1
    // They should use the line below:
    // `let answer = Part2::guess_the_number(&mut player, min, max);`
    // Make sure to test that the number of steps that part2 takes is "small"
    // Look at part1_tests for inspiration.
    #[test]
    fn the_min() {
        let min = 0; // kept the min value because I'm wildly unoriginal
        let max = 25;
        let number = min;
        let max_steps = max/2 + 1; // max steps for a binary search can be solved logathically but that seemed a bit unnecessary for this assignment so I halved the max and added one to be safe since dividing an odd number will cause the decimal to be dropped

        let mut player = Player::new(SimulatedPlayer::new(number)); // simulated player code adopted from part1
        let answer = Part2::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number); // answer checker adopted from part1
        assert!(player.steps() <= max_steps); // step checker adapted from part1
    }

    #[test]
    fn the_max() {
        let min = 0; // kept the min value because I'm wildly unoriginal
        let max = 25;
        let number = max - 1;
        let max_steps = max/2 + 1; // max steps for a binary search can be solved logathically but that seemed a bit unnecessary for this assignment so I halved the max and added one to be safe since dividing an odd number will cause the decimal to be dropped
        let mut player = Player::new(SimulatedPlayer::new(number)); // simulated player code adopted from part1
        let answer = Part2::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number); // answer checker adopted from part1
        assert!(player.steps() <= max_steps); // step checker adapted from part1
    }

    #[test]
    fn a_different_number() {
        let min = 0; // kept the min value because I'm wildly unoriginal
        let max = 25;
        let number = 13;
        let max_steps = max/2 + 1; // max steps for a binary search can be solved logathically but that seemed a bit unnecessary for this assignment so I halved the max and added one to be safe since dividing an odd number will cause the decimal to be dropped
        
        let mut player = Player::new(SimulatedPlayer::new(number)); // simulated player code adopted from part1
        let answer = Part2::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number); // answer checker adopted from part1
        assert!(player.steps() <= max_steps); // step checker adapted from part1
    }

}

