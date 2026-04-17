use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.

    fn heuristic(board: &Board) -> i32 {
        let mut est_score_x = 0;
        let mut est_score_o = 0;
        let current_score = board.score(); //
    
        if current_score > 0 { // X is favored to win
            est_score_x += current_score;
        } else if current_score < 0 { // O is favored to win
            est_score_o -= current_score;
        } 
   } 

pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // heuristic(board);
        
        // add base case if game is over
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        // board.moves() returns all possible moves w/o having to recursively check if each cell of the board 
        let moves = board.moves();

        // in minimax, X is maximizing and O is minimizing, so we need to start from the worst possible value to ensure the score gets updated correctly.
        // if we start from 0, we can miss cases where all outcomes are worse than 0 (like all -1 for X), and the algorithm would return the wrong result.
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        // initializing best_move with the first available move so we always have a valid default move to return
        let mut best_move = moves[0];

        for (x, y) in moves { // looping through move options
            
            // apply a move 
            board.apply_move((x, y), player);
            
            // recursively call solve to test the outcome of the move 
            // the score returned by the recursive call is from the opponent perspective
            // player needs to be flipped to evaluate the score from the current player's perspective
            // score value is then used below            
            let (score, _, _) = SolutionAgent::solve(board, player.flip(), _time_limit);
           
            // undo_move restores board so the next move can be tested & avoids excessive memory use from cloning 
            // function uses mutable reference to the board so it can be modified in place and restored
            // means that a mut ref doesn't need to be explicitly written in fn solve
            board.undo_move((x, y), player);

            // check if score of current move beats previous best_score from previous iterations
            // update values when optimal move is found
            match player { 
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
                Player::O => { // if the current player is O (minimizing), choose the move with the lowest score
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }
        }

        (best_score, best_move.0, best_move.1) // return the best score and the coordinates of the best move
    }
}
 
  