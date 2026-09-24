use crate::{Game, GameState, Player, TileState};

const SEARCH_DEPTH: u32 = 5;
const WIN_SCORE: i32 = 1_000_000;

pub fn get_possible_moves(board: [TileState; 42]) -> Vec<usize> {
    (1..=7)
        .filter(|&col| board[Game::convert_2d_to_1d(col - 1, 5)] == TileState::Empty)
        .collect()
}

pub fn get_best_move(board: [TileState; 42], bot: Player) -> usize {
    let me = bot.to_tile_state();
    let mut best_move = 0;
    let mut best_score = i32::MIN;

    for m in get_possible_moves(board) {
        let score = minimax(play(board, m, me), SEARCH_DEPTH - 1, false, me);
        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }
    best_move
}

fn minimax(board: [TileState; 42], depth: u32, maximizing: bool, me: TileState) -> i32 {
    let game = Game {
        turn: Player::Red,
        board,
        move_counter: 0,
    };
    if let GameState::Winner(winner) = game.check_game_state() {
        return if winner.to_tile_state() == me {
            WIN_SCORE + depth as i32
        } else {
            -WIN_SCORE - depth as i32
        };
    }

    let moves = get_possible_moves(board);
    if moves.is_empty() {
        return 0;
    }
    if depth == 0 {
        return evaluate(&board, me);
    }

    let scores = moves.into_iter().map(|m| {
        let tile = if maximizing { me } else { opponent(me) };
        minimax(play(board, m, tile), depth - 1, !maximizing, me)
    });
    if maximizing {
        scores.max().unwrap()
    } else {
        scores.min().unwrap()
    }
}

fn evaluate(board: &[TileState; 42], me: TileState) -> i32 {
    let mut score = 0;
    for col in 0..7 {
        for row in 0..6 {
            for (dc, dr) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                let end_col = col + 3 * dc;
                let end_row = row + 3 * dr;
                if end_col < 0 || end_col > 6 || end_row < 0 || end_row > 5 {
                    continue;
                }
                let mut mine = 0;
                let mut theirs = 0;
                for step in 0..4 {
                    let c = (col + step * dc) as usize;
                    let r = (row + step * dr) as usize;
                    let tile = board[Game::convert_2d_to_1d(c, r)];
                    if tile == me {
                        mine += 1;
                    } else if tile == opponent(me) {
                        theirs += 1;
                    }
                }
                score += match (mine, theirs) {
                    (3, 0) => 5,
                    (2, 0) => 2,
                    (0, 3) => -5,
                    (0, 2) => -2,
                    _ => 0,
                };
            }
        }
    }
    score
}

fn play(mut board: [TileState; 42], col: usize, tile: TileState) -> [TileState; 42] {
    let row = (0..6)
        .find(|&row| board[Game::convert_2d_to_1d(col - 1, row)] == TileState::Empty)
        .expect("Spalte ist voll");
    board[Game::convert_2d_to_1d(col - 1, row)] = tile;
    board
}

fn opponent(tile: TileState) -> TileState {
    match tile {
        TileState::Red => TileState::Yellow,
        TileState::Yellow => TileState::Red,
        TileState::Empty => panic!("Empty hat keinen Gegner"),
    }
}
