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

    fn ask_if_equal(&mut self, guess: u32) -> bool {
        
        if guess == self.the_number {
            return true; // because dur
        }
        else {
            return false;
        }
    }

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

