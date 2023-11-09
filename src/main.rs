mod pieces;
mod board;

use board::*;

fn main() -> Result<(), &'static str> {
    let mut board = Board::new();
    board.init("");
    board.r#move("A2", "A3")?;
    board.r#move("A3", "A4")?;
    board.r#move("A4", "A5")?;
    board.r#move("A5", "A6")?;
    board.r#move("A6", "B7")?;
    println!("{board}");
    Ok(())
}
