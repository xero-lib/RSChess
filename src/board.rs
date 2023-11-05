use std::fmt::Display;
use crate::pieces::{Pieces::*, Piece};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
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

    pub fn from(y: u8, x: u8) -> Self {
        if x > 7 || y > 7{
            panic!("Square indicies must be from 0 through 7, inclusive");
        }

        Self (
            ((y as u8) << 4) + x as u8,
            None 
        )
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

}

pub struct Board([Square; 64]); // optimize to a 1D array?}

impl Board {
    pub fn new() -> Self {
        let mut board = Board([Square::new(); 64]); //seems inefficient 
        
        for i in 0..8 {
            for j in 0..8 {
                *board.get_index(i, j) = Square::from(i, j);
            }
        }
        
        board
    }

    pub fn get_index(&mut self, y: u8, x: u8) -> &mut Square {
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
        if  index >= 64 {
            panic!("Invalid index parameter");
        }
        
        &mut self.0[index as usize]
    }
    
    // init pieces
    pub fn init(&mut self, _state: &str) { // impl state 
        // init pawns
        for i in 0..8 {
            // white pawns
            self.get_index(1, i).set_piece(Piece::from(Pawn, White));
            // black pawns
            self.get_index(6, i).set_piece(Piece::from(Pawn, Black));
        }

        // init rooks
        self.get_index(0, 0).set_piece(Piece::from(Rook, White));
        self.get_index(0, 7).set_piece(Piece::from(Rook, White));
        self.get_index(7, 0).set_piece(Piece::from(Rook, Black));
        self.get_index(7, 7).set_piece(Piece::from(Rook, Black));

        // init knights
        self.get_index(0, 1).set_piece(Piece::from(Knight, White));
        self.get_index(0, 6).set_piece(Piece::from(Knight, White));
        self.get_index(7, 1).set_piece(Piece::from(Knight, Black));
        self.get_index(7, 6).set_piece(Piece::from(Knight, Black));

        // init bishops
        self.get_index(0, 2).set_piece(Piece::from(Bishop, White));
        self.get_index(0, 5).set_piece(Piece::from(Bishop, White));
        self.get_index(7, 2).set_piece(Piece::from(Bishop, Black));
        self.get_index(7, 5).set_piece(Piece::from(Bishop, Black));

        //init queens
        self.get_index(0, 3).set_piece(Piece::from(Queen, White));
        self.get_index(7, 3).set_piece(Piece::from(Queen, Black));

        // init kings
        self.get_index(0, 4).set_piece(Piece::from(King, White));
        self.get_index(7, 4).set_piece(Piece::from(King, Black));
    }

    pub fn r#move(&mut self, start: &str, end: &str) -> Result<(), &str> {
        let start = Square::string_to_u8(start);
        let end = Square::string_to_u8(end);
        println!("{} {}", (start >> 4), (start & 0x0F));
        println!("{} {}", (end >> 4), (end & 0x0F));
        // check legal move here
        // std::mem::swap(&mut self.0[(end >> 4) as usize][(end & 0x0F) as usize].1, &mut self.0[(start >> 4) as usize][(start & 0x0F) as usize].1);
        // self.0[(start >> 4) as usize][(start & 0x0F) as usize].1 = None;
        let Some(piece) = self.get_index(start & 0x0F, start >> 4).get_piece() else { return Err("No piece at start") };
        self.get_index(end & 0x0F, end >> 4).set_piece(piece);
        self.get_index(start & 0x0F, start >> 4).remove_piece();
        Ok(())
    }

    pub fn get_state(&self) -> [Square; 64] {
        self.0
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
