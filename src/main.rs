
use std::{io::stdin, path::Display};

mod game;
use game::{Game, Player, CellState};

fn main() {
    println!("Hello, welcome in the Nought and Crosses game!");
    println!("Enter the name of the Player 1:");
    let mut name_player1 = String::new();
    stdin().read_line(&mut name_player1).expect("Failed to read line");
    println!("Enter the name of the Player 2:");
    let mut name_player2 = String::new();
    stdin().read_line(&mut name_player2).expect("Failed to read line");
    let mut game = Game::new(name_player1, name_player2);
    println!("You will play with entering a X point and a Y point to place your symbol on the grid. The grid is starting from 1 to 3 in absyss and 1 to 3 in ordonate");
    let mut turn = 0;
    while game.winner == String::new() {
        // display_board(&game.board);
        print!("BOARD\n{}\n", game.board);
        let player = &game.player[turn%2];
        print!("Player {}, it's your turn! ", player.name);
        let mut x = String::new();
        let mut y = String::new();
        println!("Enter a X position :");
        stdin().read_line(&mut x).expect("Failed to read line");
        println!("Enter a Y position :");
        stdin().read_line(&mut y).expect("Failed to read line");
        let x = match x.trim().parse::<usize>() {
            Ok(r) => r-1,
            Err(_) => { println!("Error while trying to parse X"); continue; }
        };
        let y = match y.trim().parse::<usize>() {
            Ok(r) => r-1,
            Err(_) => { println!("Error while trying to parse Y"); continue; }
        };
        if !(0..3).contains(&x) || !(0..3).contains(&y) {
            println!("The point is out of range from the board");
            continue
        }
        if game.board.0[x][y] != CellState::Empty {
            println!("The point is already set");
            continue
        } else {
            game.board.0[x][y] = CellState::Symbol(player.symbol);
        }
        if game.is_win() {
            game.winner = player.name.clone();
        }
        turn = turn+1;
    }
    print!("BOARD\n{}", game.board);
    println!("Player '{}' win the game!", game.winner)
}

