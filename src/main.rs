use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{self, Write};

use rand::random_bool;
mod bot;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn to_tile_state(&self) -> TileState {
        match self {
            Player::Red => TileState::Red,
            Player::Yellow => TileState::Yellow,
        }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Player::Yellow => String::from("Yellow"),
            Player::Red => String::from("Red"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileState {
    Red,
    Yellow,
    Empty,
}

impl TileState {
    pub fn to_player(&self) -> Player {
        match self {
            TileState::Yellow => Player::Yellow,
            TileState::Red => Player::Red,
            Self::Empty => {
                panic!("Can't call get player on empty tile.")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameError {
    RowIsFull,
    InvalidInput,
    OutOfRange,
}

struct Game {
    turn: Player,
    board: [TileState; 42],
    move_counter: u8,
}

enum GameState {
    Winner(Player),
    Draw,
    Ongoing,
}
impl Game {
    pub fn create() -> Self {
        let board = [TileState::Empty; 42];
        let move_counter = 0;
        let turn = if random_bool(0.5) {
            Player::Red
        } else {
            Player::Yellow
        };
        println!("Created new game. Starting player: {:?}", turn);
        Game {
            turn,
            board,
            move_counter,
        }
    }
    pub fn clear_terminal() {
        io::stdout().execute(Clear(ClearType::All)).unwrap();
        io::stdout().flush().unwrap();
    }
    pub fn start(&mut self) {
        loop {
            self.print_board();

            loop {
                // Bots turn
                let m = if self.turn == Player::Yellow {
                    let m = bot::get_best_move(self.board, self.turn);
                    m
                }
                // Players turn
                else {
                    let input = match self.get_input("Move: ") {
                        Ok(i) => i,
                        Err(e) => {
                            self.print_board();
                            println!("Fehler bei der Eingabe: {:?}", e);
                            continue;
                        }
                    };
                    input
                };

                if let Err(e) = self.make_move(m) {
                    self.print_board();
                    println!("Fehler beim Zug: {:?}", e);
                    continue;
                }

                break;
            }

            match self.check_game_state() {
                GameState::Draw => {
                    self.print_board();
                    println!("Its a draw..");
                    break;
                }
                GameState::Ongoing => {}
                GameState::Winner(player) => {
                    self.print_board();
                    println!("Player {} has won.", player.to_string());
                    break;
                }
            }
            self.change_player();
        }
    }
    pub fn check_game_state(&self) -> GameState {
        for r in 0..6 {
            let row: Vec<&TileState> = self.board.iter().skip(r).step_by(6).collect();
            for window in row.windows(4) {
                if window
                    .iter()
                    .all(|&x| x == window[0] && *x != TileState::Empty)
                {
                    return GameState::Winner(window[0].to_player());
                }
            }
        }
        for col in self.board.chunks(6) {
            for window in col.windows(4) {
                if window
                    .iter()
                    .all(|&item| item == window[0] && item != TileState::Empty)
                {
                    return GameState::Winner(window[0].to_player());
                }
            }
        }
        for i in 0..self.board.iter().len() {
            for offset in [5, 7] {
                // 5 und 7 sind die diagonale offsets
                if (offset == 7 && i % 6 > 2) || (offset == 5 && i % 6 < 3) {
                    continue;
                }
                let mut diag = Vec::new();
                for step in 0..4 {
                    let val = self.board.get(i + step * offset); // 1 - 4 mal in die diagonale
                                                                 // gehen mit dem offset
                    if let Some(v) = val
                        && val != Some(&TileState::Empty)
                    {
                        // es gibt dort ein feld (nicht out of bounds) und es ist nicht leer
                        diag.push(v);
                    } else {
                        break;
                    }
                }
                if diag.iter().len() == 4 {
                    // es wurden 4 in eine diagnonale gefunden
                    if diag.iter().all(|&x| x == diag[0]) {
                        return GameState::Winner(diag[0].to_player());
                    }
                }
            }
        }

        if self.move_counter == 42 {
            return GameState::Draw;
        }

        GameState::Ongoing
    }
    pub fn print_board(&self) {
        Game::clear_terminal();
        println!("\n 1 2 3 4 5 6 7"); // Spaltennummern
        println!("┌───────────────┐");

        // 6 Reihen von oben nach unten (row 5 bis 0)
        for row in (0..6).rev() {
            print!("|");
            for col in 0..7 {
                let tile = self.board[Game::convert_2d_to_1d(col, row)];
                let symbol = match tile {
                    TileState::Empty => ".",
                    TileState::Red => "R",
                    TileState::Yellow => "Y",
                };
                print!("{} ", symbol);
            }
            println!("|");
        }

        println!("└───────────────┘\n");
    }
    pub fn change_player(&mut self) {
        if self.turn == Player::Red {
            self.turn = Player::Yellow;
        } else {
            self.turn = Player::Red;
        }
    }
    pub fn get_input(&self, prompt: &str) -> Result<usize, GameError> {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Fehler beim Lesen");
        let input = input.trim();
        let row = input
            .parse::<usize>()
            .map_err(|e| GameError::InvalidInput)?;

        if row <= 0 || row > 7 {
            return Err(GameError::OutOfRange);
        }
        Ok(row)
    }
    pub fn convert_1d_to_2d(pos: usize) -> (usize, usize) {
        let row = pos / 6;
        let col = pos % 6;
        (row, col)
    }

    pub fn convert_2d_to_1d(row: usize, col: usize) -> usize {
        row * 6 + col
    }

    pub fn make_move(&mut self, mut row: usize) -> Result<(), GameError> {
        if (row < 1 || row > 7) {
            // der Input wird  schon minus 1
            return Err(GameError::OutOfRange);
        }
        row -= 1;
        let mut col: usize = 0;

        // Checken ob das oberste in der Reihe überhaupt frei ist. Falls dies bereits belegt ist
        // kann der move nicht valid sein, sonst muss er valid sein.
        if self.board[Game::convert_2d_to_1d(row, 5)] != TileState::Empty {
            return Err(GameError::RowIsFull);
        }

        while self.board[Game::convert_2d_to_1d(row, col)] != TileState::Empty {
            col += 1; // Ganz unten starten und immer eine Zeile hoch gehen, bis eine leere
                      // gefunden wurde
        }

        self.board[Game::convert_2d_to_1d(row, col)] = self.turn.to_tile_state();

        self.move_counter += 1;
        Ok(())
    }
}

fn main() {
    let mut game = Game::create();
    game.start();
}
