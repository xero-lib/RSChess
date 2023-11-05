use crate::board::Color::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

#[derive(Debug)]
pub enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}


use Pieces::*;
// use phf::phf_map;

// static OFFSETS: phf::Map<&'static Pieces, &'static [(i8, i8)]> = phf_map! {
//     // all offsets are listed in (y, x) format
//     Pawn => [(0, 1)]
// };

#[derive(Clone, Copy)]
pub struct Piece {
    pub r#type: Pieces,
    pub color: Color,
}

impl Piece {
    pub fn from(r#type: Pieces, color: Color) -> Self {
        Piece { r#type, color }
    }

    pub fn get_char(&self) -> char {
        let letter = match self.r#type {
            Pawn => 'p',
            Rook => 'r',
            Knight => 'n',
            Bishop => 'b',
            Queen => 'q',
            King => 'k',
        };

        match self.color {
            White => letter.to_ascii_uppercase(),
            _ => letter,
        }
    }
}

impl Piece {
    pub const OFFSETS_PAWN: [(i8, i8); 8] = [
        (0,  1),
        (0,  2),
        (1,  1),
        (-1, 1),

        (0,  -1),
        (0,  -2),
        (1,  -1),
        (-1, -1),
    ];

    pub const OFFSETS_ROOK: [(i8, i8); 28] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
        (0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7),
        (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0),
    ];

    pub const OFFSETS_KNIGHT: [(i8, i8); 8] = [
        (2, 1), (-2, 1),
        (1, 2), (-1, 2),
        (2, -1), (-2, -1),
        (1, -2), (-1, -2),
    ];

    pub const OFFSETS_BISHOP: [(i8, i8); 28] = [
        (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7),
        (-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7),
        (1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7),
        (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7),
    ];

    pub const OFFSETS_QUEEN: [(i8, i8); 56] = [ // Bishop + Rook
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
        (0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7),
        (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0),
        (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7),
        (-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7),
        (1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7),
        (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7),
    ];
    
    pub const OFFSETS_KING: [(i8, i8); 8] = [
        (-1, 1), (0, 1), (1, 1),
        (-1, 0),          (1, 0),
        (-1, -1), (0, -1), (1, -1),
    ];
}


/*
    to calc if a move results in self check, simply search in the straight and diagonal directions and stop when you hit a piece or an edge
    if the piece intersected is the same color, that direction does not check.
    else if the piece intersected is a bishop (for diagonal) or a rook (for straight) or a queen (for both) of the opposite color, it is a check
*/