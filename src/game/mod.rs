use std::fmt::Display;

pub struct Game {
    pub board: Board,
    pub player: [Player; 2],
    pub winner: String,
}

impl Game {
    pub fn new(player1: String, player2: String) -> Game {
        Game {
            board: Board::new(),
            player: [
                Player {
                    name: player1,
                    symbol: Symbol::Cross,
                },
                Player {
                    name: player2,
                    symbol: Symbol::Circle,
                },
            ],
            winner: String::new(),
        }
    }
    pub fn launch_game(&self) {
        println!("yo")
    }
    // pub fn play(&mut self, x: usize, y: usize) {
    //     let player = &self.player[0];
    //     self.board.0[x][y] = CellState::Symbol(player.symbol);
    //     if self.is_win() {
    //         self.winner = player.name;
    //     }
    // }
    pub fn is_win(&self) -> bool{
        let board = &self.board.0;
        let diagonal1 = CellState::Empty != board[0][0] && board[0][0] == board[1][1] && board[1][1] == board[2][2];
        let diagonal2 = CellState::Empty != board[0][2] && board[0][2] == board[1][1] && board[1][1] == board[2][0];
        if diagonal1 || diagonal2 {
            return true
        }
        for i in 0..board.len() {
            let line = CellState::Empty != board[i][0] && board[i][0] == board[i][1] && board[i][1] == board[i][2];
            let column = CellState::Empty != board[0][i] && board[0][i] == board[1][i] && board[1][i] == board[2][i];
            if line || column {
                return true
            }
        }
        false
    }
}

// pub struct Player {
//     pub name: String,
//     pub symbol: char,
// }

pub struct Player {
    pub name: String,
    pub symbol: Symbol,
}

pub struct Board(pub [[CellState; 3]; 3]);

impl Board {
    fn new() -> Board {
        Board([[CellState::Empty; 3]; 3])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if j > 0 && j < self.0.len(){
                    write!(f, "|")?;
                }
                write!(f, "{}", cell)?;
            }
            if i + 1 != self.0.len() {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Symbol {
    Circle,
    Cross
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Symbol(Symbol)
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, " "),
            CellState::Symbol(Symbol::Circle) => write!(f, "O"),
            CellState::Symbol(Symbol::Cross) => write!(f, "X"),
        }
    }
}

// struct Position {
//     x: u8,
//     y: u8,
// }

// struct board<T> {
//     board: Vec<Vec<T>>,
// }

// impl<T: Display> board<T> {
//     fn display(&self) {
//         println!("yo")
//     }
// }