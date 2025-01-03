use std::{fmt::Display, ops::Not};
use crate::pieces::{Pieces::*, Piece};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Color {
    Black,
    White,
}

impl Not for Color {
    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::White,
        }
    }

    type Output = Self;
}

use Color::*;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Black => "Black",
            Self::White => "White"
        })
    }
}

#[derive(Clone, Copy)]
pub struct Square(u8, Option<Piece>);

impl Square {
    pub fn new() -> Self { // only used to initialize board
        Self(0, None)
    }

    pub fn from(x: u8, y: u8) -> Self {
        if x > 7 || y > 7{
            panic!("Square indicies must be from 0 through 7, inclusive");
        }

        Self (
            ((x as u8) << 4) + y as u8,
            None 
        )
    }

    pub fn get_coords_u8(&self) -> u8 {
        self.0
    }

    pub fn get_coords_tuple(&self) -> (u8, u8) {
        (self.0 >> 4, self.0 & 0x0F)
    }

    #[allow(unused)]
    pub fn get_coords_string(&self) -> String {
        format!("{}{}", ((self.0 >> 4) as u8 + b'A') as char, (self.0 & 0x0F) + 1)
    }

    pub fn get_color(&self) -> Color {
        return if (((self.0 & 0xF0) >> 4) ^ (self.0 & 0x0F)) & 1 == 0 {
           Black
        } else {
           White
        }
    }

    pub fn get_piece(&self) -> Option<Piece> {
        self.1.clone()
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.1 = Some(piece);
    }

    pub fn remove_piece(&mut self) {
        self.1 = None;
    }

    pub fn get_char(&self) -> char {
        match self.get_piece() {
            Some(piece) => piece.get_char(),
            None => match self.get_color() {
                Black => '#',
                White => ' ',
            },
        }
    }
}

impl Square {
    pub fn string_to_u8(coord: &str) -> u8 {
        if coord.len() != 2 {
            panic!("Coordinate string must be exactly 2 characters: A1, B2, etc");
        }

        let lower = coord.to_ascii_lowercase();
        let mut chars = lower.chars();
        let c = chars.next().unwrap();
        if c as u8 - b'a' > 8 {
            panic!("Invalid coordinate entry to string_to_u8: {coord}");
        }

        let r = chars.next().unwrap();

        (c as u8 - b'A' << 4) + r.to_digit(10).expect("Invalid coord string") as u8 - 1
    }

    pub fn string_to_tuple(coord: &str) -> (u8, u8) {
        if coord.len() != 2 {
            panic!("Coordinate string must be exactly 2 characters: A1, B2, etc");
        }
        let coord = coord.to_ascii_lowercase();
        let mut chars = coord.chars();
        let [c, r]  = [chars.next().unwrap(), chars.next().unwrap()];

        (c as u8 - b'a', r as u8 - b'1')
    }

    pub fn u8_to_tuple(coord: u8) -> (u8, u8) {
        (coord >> 4, coord & 0x0F)
    }

}

#[derive(Clone, Copy)]
pub struct Board([Square; 64], u16); // (Board datastructure, enpassantable: black left 8 bits, white right 8)

impl Board {
    pub fn new() -> Self {
        let mut board = Board([Square::new(); 64], 1); //seems inefficient 
        
        for i in 0..8 {
            for j in 0..8 {
                *board.get_index_mut(i, j) = Square::from(i, j);
            }
        }
        
        board
    }

    pub fn get_index_mut(&mut self, x: u8, y: u8) -> &mut Square {
        /*
            [
                 0,  1,  2,  3,  4,  5,  6,  7,
                 8,  9, 10, 11, 12, 13, 14, 15,
                16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31,
                32, 33, 34, 35, 36, 37, 38, 39,
                40. 41. 42. 43. 44. 45, 46, 47,
                48, 49, 50, 51, 52, 53, 54, 55,
                56, 57, 58, 59, 60, 61, 62, 63,  
            ]

            y * 8 + x
         */
        let index = y * 8 + x;
        if  index > 63 {
            panic!("Invalid index parameter");
        }
        
        &mut self.0[index as usize]
    }
    
    pub fn get_index(&self, x: u8, y: u8) -> &Square {
        /*
            [
                 0,  1,  2,  3,  4,  5,  6,  7,
                 8,  9, 10, 11, 12, 13, 14, 15,
                16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31,
                32, 33, 34, 35, 36, 37, 38, 39,
                40. 41. 42. 43. 44. 45, 46, 47,
                48, 49, 50, 51, 52, 53, 54, 55,
                56, 57, 58, 59, 60, 61, 62, 63,  
            ]

            y * 8 + x
         */
        let index = y * 8 + x;
        if  index > 63 {
            panic!("Invalid index parameter");
        }
        
        &self.0[index as usize]
    }
    // init pieces
    pub fn init(&mut self, _state: &str) { // impl state 
        // init pawns
        for i in 0..8 {
            self.get_index_mut(i, 1).set_piece(Piece::from(Pawn, White)); // white pawns
            self.get_index_mut(i, 6).set_piece(Piece::from(Pawn, Black)); // black pawns
        }

        // init rooks
        self.get_index_mut(0, 0).set_piece(Piece::from(Rook, White));
        self.get_index_mut(7, 0).set_piece(Piece::from(Rook, White));
        self.get_index_mut(0, 7).set_piece(Piece::from(Rook, Black));
        self.get_index_mut(7, 7).set_piece(Piece::from(Rook, Black));

        // init knights
        self.get_index_mut(1, 0).set_piece(Piece::from(Knight, White));
        self.get_index_mut(6, 0).set_piece(Piece::from(Knight, White));
        self.get_index_mut(1, 7).set_piece(Piece::from(Knight, Black));
        self.get_index_mut(6, 7).set_piece(Piece::from(Knight, Black));

        // init bishops
        self.get_index_mut(2, 0).set_piece(Piece::from(Bishop, White));
        self.get_index_mut(5, 0).set_piece(Piece::from(Bishop, White));
        self.get_index_mut(2, 7).set_piece(Piece::from(Bishop, Black));
        self.get_index_mut(5, 7).set_piece(Piece::from(Bishop, Black));

        //init queens
        self.get_index_mut(3, 0).set_piece(Piece::from(Queen, White));
        self.get_index_mut(3, 7).set_piece(Piece::from(Queen, Black));

        // init kings
        self.get_index_mut(4, 0).set_piece(Piece::from(King, White));
        self.get_index_mut(4, 7).set_piece(Piece::from(King, Black));
    }

    pub fn r#move(&mut self, start: &str, end: &str) -> Result<(), &'static str> {
        let (start_x, start_y) = Square::string_to_tuple(start);
        let Some(piece) = self.get_index(start_x, start_y).get_piece() else { return Err("No piece at start") };
        if piece.get_moves(&mut self.clone(), &self.get_index(start_x, start_y)).contains(&Square::string_to_u8(end)) {
            let (x, y) = Square::string_to_tuple(end); // target square x and y
            self.get_index_mut(x, y).set_piece(piece);

            // check/set pawn en passantable
            self.clear_epassantable(None);
            if y == 3 && start_y == 1 && piece.r#type == Pawn {
                self.set_enpassantable(piece.color, x);
            }

            let (x, y) = self.get_index(start_x, start_y).get_coords_tuple();
            self.get_index_mut(x, y).remove_piece();
            return Ok(())
        }
        return Err("Illegal move")
        // check legal move here
        // std::mem::swap(&mut self.0[(end >> 4) as usize][(end & 0x0F) as usize].1, &mut self.0[(start >> 4) as usize][(start & 0x0F) as usize].1);
        // self.0[(start >> 4) as usize][(start & 0x0F) as usize].1 = None;
    }

    pub fn get_state(&self) -> [Square; 64] {
        self.0
    }

    pub fn get_enpassantable(&self, color: Color) -> Vec<u8> {
        let mut squares = match color {
            Black => self.1 as u8, // drop leftmost 8 bits
            White => (self.1 >> 8) as u8 // shift leftmost 8 bits to be rightmost 8 bits, drop empty bits
        };

        let mut out: Vec<u8> = Vec::new();
        for i in 0..8 {
            if squares & 1 == 1 { out.push(i); }
            squares >>= 1;
        }

        out
    }

    pub fn set_enpassantable(&mut self, color: Color, x: u8) {
        let bit = 2u16.pow(match color {
            Black => x,
            White => x + 8
        } as u32);
        if self.1 & bit == bit {
            unreachable!("Requested enpassant square is already enpassantable. This sould be impossible.");
        }
        self.1 += bit;
    }

    pub fn clear_epassantable(&mut self, color: Option<Color>) {
        match color {
            // clear bits
            Some(Black) => self.1 &= !0xFF00,
            Some(White) => self.1 &= !0x00FF,
            None => self.1 = 0
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  ╔═════════════════╗  ")?;
        for y in 0..8 {
            write!(f, "{} ║ ", 8 - y)?;
            for x in 0..8 {
                write!(f, "{} ", self.get_state()[(7 - y) * 8 + x].get_char())?;
            }
            write!(f, "║\n")?;
        }
        writeln!(f, "  ╚═════════════════╝\n    A B C D E F G H")?;
        Ok(())
    }
}
