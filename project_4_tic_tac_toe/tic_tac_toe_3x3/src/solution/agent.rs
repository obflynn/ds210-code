use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}
  

// Put your solution here.
impl Agent for SolutionAgent {
        fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
      // add base case if game is over
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves(); //We can use board.moves() here instead of looping through the grid cause it already gives all available moves and makes the code cleaner
//I think the recursion could return incorrect results while this version guarantees correct evaluation of all possible game outcomes

// changed mut score to MIN and Max instead of setting both to 0
//In minimax, X is maximizing and O is minimizing, so we need to start from the worst possible value to ensure the score gets updated correctly.
//If we start from 0, we can miss cases where all outcomes are worse than 0 (like all -1 for X), and the algorithm would return the wrong result.
let mut best_score = match player {
            Player::X => i32::MIN, // maximize if X
            Player::O => i32::MAX, // minimize if O
        };

        let mut best_move = moves[0]; // Initializing best_move with the first available move so we always have a valid default in case no better move is found
        // stores the best position (x, y) that algorithm has found so far
        // Track best_move separately so we return the correct position associated with the best score

        for (x, y) in moves {
            // need to clone board so we do not modify the current one
            let mut new_board = board.clone();
            new_board.apply_move((x, y), player);

            //Switch player for the recursive call
            let next_player = match player {
                Player::X => Player::O,
                Player::O => Player::X,
            };

            let (score, _, _) = SolutionAgent::solve(&mut new_board, next_player, _time_limit); //Call solve recursively to evaluate the outcome of this move

            // If the current player is X (maximizing), choose the move with the highest score
            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                } // If the current player is O (minimizing), choose the move with the lowest score
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }
        }

        (best_score, best_move.0, best_move.1) //return the best score and the coordinates of the best move
    }
}
 
  