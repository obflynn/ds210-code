use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{self, Board};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {
    player: Player, // the program
    //player_opp: Player, // the opponent
    //board: Board, // the current board state
}

impl std::ops::Deref for SolutionAgent { 
    type Target = Player; 

    fn deref(&self) -> &Self::Target {  
        &self.player
    }
}
    // Board Set-Up:
    //     0:  1:  2:
    // 0:  _ | _ | _
    // 1:  _ | X | _
    // 2:  _ | _ | _
    // board.get_cells()[1][1] would return middle cell (aka Cell::X in this case)


// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.

    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(board, player, _time_limit)

       let mut score = 0;
       let mut x = 0;
       let mut y = 0;   
       let board_cells = board.get_cells();
       let board_size = board_cells.len();
       
       
       for i in 0..board_size { // iterate through rows
           
           for j in 0..board_size { // iterate through columns
               
               if board_cells[i][j] == board::Cell::Empty { // if a cell is empty, then a move can be made there
                   
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

