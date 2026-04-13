use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{self, Board};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}
  
// Board Set-Up:
    //     0:  1:  2:
    // 0:  _ | _ | _
    // 1:  _ | X | _
    // 2:  _ | _ | _
    // board.get_cells()[1][1] would return middle cell (aka Cell::X in this case)

    
impl Agent for SolutionAgent {
   
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {

       let mut score = 0;
       let mut new_score = 0;
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
       
       (new_score, x, y) //return the best score and the coordinates of the best move
    }
}
 