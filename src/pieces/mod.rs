use nalgebra::base::Vector2;

pub trait Piece {
    //Rook (R), Knight(N), Bishop(B), Queen(Q), King(K), Pawn(P),

    // We need to use &self here as we can only include functions in a vtable if they require themself
    /// Returns the char that is used to display the piece
    fn get_char(&self) -> char;

    /// Gets called when the piece gets moved,
    /// to replace the current piece with another return Some, otherwise None
    fn update(&self, position: Vector2<isize>) -> Option<Box<dyn Piece>>;

    fn possible_moves(&self, position: Vector2<isize>) -> Vec<Vector2<isize>>;
}
