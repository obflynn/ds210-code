use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    // just a placeholder so it's easier to work back and forth
    fn heuristic(board: &Board) -> i32 {
        board.score()
    }

    // depth-limited minimax
    fn minimax(board: &mut Board,player: Player, depth: usize, max_depth: usize, ) -> (i32, usize, usize) {

        //checks whether the game has already ended
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        //checks whether the search has reached the maximum allowed depth
        if depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
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