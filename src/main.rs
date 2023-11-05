mod pieces;
mod board;

use std::io::Error;

use board::*;

fn main() -> Result<(), Error>{
    let mut board = Board::new();
    println!("{board}");
    board.init("");
    println!("{board}");
    board.r#move("A2", "A3").unwrap();
    println!("{board}");
    Ok(())
}
