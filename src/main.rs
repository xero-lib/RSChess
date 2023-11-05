mod pieces;
mod board;

use board::*;

fn main() -> Result<(), &'static str>{
    let mut board = Board::new();
    println!("{board}");
    board.init("");
    println!("{board}");
    board.r#move("A2", "A3")?;
    println!("{board}");
    board.r#move("A3", "A4")?;
    print!("{board}");
    board.r#move("A4", "B5")?;
    Ok(())
}
