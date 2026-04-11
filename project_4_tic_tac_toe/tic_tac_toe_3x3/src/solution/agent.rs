use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {
    player: Player, // the program
    //player_opp: Player, // the opponent
    //board: Board, // the current board state
}
//I'm not quite sure what this part is so I'm gonna comment it out first
/* impl std::ops::Deref for SolutionAgent { 
    //type Target = Player; 

    //fn deref(&self) -> &Self::Target {  
       // &self.player
    //}
}*/
    // Board Set-Up:
    //     0:  1:  2:
    // 0:  _ | _ | _
    // 1:  _ | X | _
    // 2:  _ | _ | _
    // board.get_cells()[1][1] would return middle cell (aka Cell::X in this case)


// Put your solution here.
impl Agent for SolutionAgent {
        fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.

      // add base case if game is over
        if board.game_over() {
            return (board.score(), 0, 0);
        }

// changed mut score to MIN and Max instead of setting both to 0
//In minimax, X is maximizing and O is minimizing, so we need to start from the worst possible value to ensure the score gets updated correctly.
//If we start from 0, we can miss cases where all outcomes are worse than 0 (like all -1 for X), and the algorithm would return the wrong result.

       let mut score = match player {
            Player::X => i32::MIN, // maximize if X
            Player::O => i32::MAX, // minimize if O
        };

       let mut x = 0;
       let mut y = 0;   
       let board_cells = board.get_cells();
       let board_size = board_cells.len();
       
       
       for i in 0..board_size { // iterate through rows
           
           for j in 0..board_size { // iterate through columns
               
               if board_cells[i][j] == Cell::Empty {// if a cell is empty, then a move can be made there
                   
                   let mut copy_board = board.clone();
                   copy_board.apply_move((i, j), player);
                   let new_score: i32 = SolutionAgent::solve(&mut copy_board, player, _time_limit).0; 
                  
                   if player == Player::X {
                       if new_score > score { // player X wants to maximize the score
                           score = new_score; // update score
                           x = i; // update x so the move is made in the correct row (i)
                           y = j; // update y so the move is made in the correct column (j)
                       }
                   } 
                   else { // player == Player::O
                       if new_score < score { // player O wants to minimize the score
                           score = new_score;
                           x = i;
                           y = j;
                       }
                   }
               }
           }
       }
       
    
       return (score, x, y); 
    }
}

