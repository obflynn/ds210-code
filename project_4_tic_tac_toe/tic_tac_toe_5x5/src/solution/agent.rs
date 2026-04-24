use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    // just a placeholder so it's easier to work back and forth
      
    fn heuristic(board: &Board, player: Player) -> i32 {
        // board.score adds(X)/subtracts(O) 1 for each possible row/column/diagonal win
        // only rewards for 3 in a row w/o taking into account 2 in a row + empty spot or blocking the opponent
        let spaces = board.get_cells();
        let n = spaces.len(); 
        let mut score = board.score() * 100;

        match player { // match score to agent's perspective (positive score = good for X, negative score = good for O)
            Player::X => score, 
            Player::O => -score, 
        };

        let mid_board = n/2;
        match spaces[mid_board][mid_board] { // favor center control and alter X/O score accordingly 
            Cell::X => score += 10, 
            Cell::O => score -= 10, 
            _ => {},
        }

        for i in 0..n {
            for j in 0..n {
                
                let mut x = 0;
                let mut o = 0;
                let mut free = 0;
                

                if j+2 < n { // check rows for 2 in a row + empty spot
                    for space in [&spaces[i][j], &spaces[i][j+1], &spaces[i][j+2]] {
                        match space {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => free += 1,
                            Cell::Wall => {} // case for cell wall that should hopefully never be reached
                        }
                    }
                    if x == 2 && free == 1 { // favor 2 in a row for X + empty spot by increasing score
                        score += 5;
                    } 
                    else if o == 2 && free == 1 { // disfavor opponent's 2 in a row + empty spot
                        score -= 5;
                    }
                }

                if i+2 < n { // check columns for 2 in a row + empty spot
                    for space in [&spaces[i][j], &spaces[i+1][j], &spaces[i+2][j]] {
                        match space {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => free += 1,
                            Cell::Wall => {} // case for cell wall that should hopefully never be reached
                        }
                    }
                    if x == 2 && free == 1 { // favor 2 in a row for X + empty spot by increasing score
                        score += 5;
                    } 
                    else if o == 2 && free == 1 { // favor 2 in a row for O + empty spot by decreasing score
                        score -= 5;
                    }
                }

                if j+2 < n && i+2 < n { // check R-diagonal for 2 in a row + empty spot
                    for space in [&spaces[i][j], &spaces[i+1][j+1], &spaces[i+2][j+2]] {
                        match space {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => free += 1,
                            Cell::Wall => {} // case for cell wall that should hopefully never be reached
                        }
                    }
                    if x == 2 && free == 1 {  // favor 2 in a row for X + empty spot by increasing score
                        score += 5;
                    } 
                    else if o == 2 && free == 1 { // favor 2 in a row for O + empty spot by decreasing score
                        score -= 5;
                    }
                }

                if j >= 2 && i+2 < n { // check L-diagonal for 2 in a row + empty spot
                    for space in [&spaces[i][j], &spaces[i+1][j-1], &spaces[i+2][j-2]] {
                        match space {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => free += 1,
                            Cell::Wall => {} // case for cell wall that should hopefully never be reached
                        }
                    }
                    if x == 2 && free == 1 {  // favor 2 in a row for X + empty spot by increasing score
                        score += 5;
                    } 
                    else if o == 2 && free == 1 { // favor 2 in a row for O + empty spot by decreasing score
                        score -= 5;
                    }
                }
            }
        }

        return score; // (+) if X is favored, (-) if O is favored, (0) if a draw is likely

    }

    // depth-limited minimax
    fn minimax(board: &mut Board,player: Player, depth: usize, max_depth: usize, ) -> (i32, usize, usize) {

        //checks whether the game has already ended
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        //checks whether the search has reached the maximum allowed depth
        if depth == max_depth {
            return (SolutionAgent::heuristic(board, player), 0, 0); 
        }

        let moves = board.moves();

        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        let mut best_move = moves[0];

        for (x, y) in moves {
            board.apply_move((x, y), player);

            let (score, _, _) =
                SolutionAgent::minimax(board, player.flip(), depth + 1, max_depth);

            board.undo_move((x, y), player);

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // if 3x3 do full search, but depth-limited if 5x5
        let max_depth = if board.moves().len() <= 9 { 9 } else { 3 }; 

        SolutionAgent::minimax(board, player, 0, max_depth) 
    }
}