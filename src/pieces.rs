use crate::board::{Color::{self, *}, Board, Square};

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

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub r#type: Pieces,
    pub color: Color,
}

impl Piece {
    pub fn from(r#type: Pieces, color: Color) -> Self {
        Piece { r#type, color }
    }

    fn get_offsets(&self) -> Vec<(i8, i8)> { // maybe convert this to impl Iterator<Item = &(i8, i8)> ?
        match self.r#type {
            Pawn => {
                match self.color {
                    Black => &Piece::OFFSETS_PAWN[4..],
                    White => &Piece::OFFSETS_PAWN[..4],
                }
            },
            Rook   => &Piece::OFFSETS_ROOK  [..],
            Knight => &Piece::OFFSETS_KNIGHT[..],
            Bishop => &Piece::OFFSETS_BISHOP[..],
            Queen  => &Piece::OFFSETS_QUEEN [..],
            King   => &Piece::OFFSETS_KING  [..],
        }.into()
    }

    pub fn get_moves(&self, board: &mut Board, coord: &Square) -> Vec<u8> {
        let mut offsets = self.get_offsets();
        let (px, py) = Square::u8_to_tuple(coord.get_coords_u8());
        
        // filter offsets that would move off of the board
        offsets.retain(|(x, y)| 
            px as i8 + x >= 0 && px as i8 + x <= 7 &&
            py as i8 + y >= 0 && py as i8 + y <= 7        
        );

        // filter offsets where a piece of the same color already exists
        offsets.retain(|(x, y)| { // problem 
            return !board.get_index((px as i8 + x) as u8, (py as i8 + y) as u8).get_piece().is_some_and(|piece| piece.color == self.color);
        });

        // instead of a has_moved property, just check if the pawn is on that colors starting square
        // remove pawn captures if there's no piece 
        if self.r#type == Pawn {
            // check if move is an en passant
            offsets.retain(|(x, y)| {
                if *x == 0 { return true; }
                return board.get_index((px as i8 + x) as u8, (py as i8 + y) as u8).get_piece().is_some_and(|p| p.color != self.color);
            });

            match board.get_enpassantable(!self.color) {
                x if px < 7 && x.contains(&(px + 1)) => offsets.push((1, 1)),
                x if px > 0 && x.contains(&(px - 1)) => offsets.push((1, -1)),
                _ => ()
            };
        }

        // remove invalid vertical and horizontal offsets
        if matches!(self.r#type, Pawn | Rook | King | Queen) {
            // horizontal
            offsets.retain(|(x, y)| {
                if *y != 0 { return true }

                for i in x.signum()..*x {
                    if board.get_index((px as i8 + i) as u8, py).get_piece().is_some() {
                        return false;
                    }
                }

                return true;
            });

            // vertical
            offsets.retain(|(x, y)| {
                if *x != 0 { return true } // is this necessary?

                for i in y.signum()..*y {
                    if board.get_index(px, (py as i8 + i) as u8).get_piece().is_some() {
                        return false;
                    }
                }

                return true;
            });
        }



        // remove invalid diagonal offsets
        if matches!(self.r#type, King | Queen | Bishop) {
            offsets.retain(|(mut x, mut y)| {
                if x.abs() != y.abs() { return true }

                // x and y minus respective signums to 0
                while x != x.signum() {                
                    if board.get_index((px as i8 + (x - x.signum())) as u8 , (py as i8 + (y - y.signum())) as u8).get_piece().is_some() {
                        return false;
                    }

                    x -= x.signum();
                    y -= y.signum();
                }

                return true;
            });
        }

        if self.r#type == Pawn {
            offsets.retain(|(x, y)| *x != 0 || !board.get_index((px as i8 + x) as u8, (py as i8 + y) as u8).get_piece().is_some());
        }

        // check if any move results in check
        offsets.retain(|(x, y)| {
            let mut ephemeral_board = board.clone();
            // generate move from offset
            let start_piece = ephemeral_board.get_index(px, py).get_piece().unwrap();
            ephemeral_board.get_index_mut((px as i8 + x) as u8, (py as i8 + y) as u8).set_piece(start_piece);
            ephemeral_board.get_index_mut(px, py).remove_piece();

            let king = ephemeral_board.get_state().into_iter().find(|square| 
                square.get_piece().is_some_and(|piece| piece.r#type == King && piece.color == self.color)
            ).expect("Unable to find king");

            let (kx, ky) = Square::u8_to_tuple(king.get_coords_u8());

            // search down 
            let mut offset: i8 = 1;
            // check for king
            if ky > 0 && ephemeral_board.get_index(kx, (ky as i8 - offset) as u8).get_piece().is_some_and(|piece| piece.r#type == King) {
                return false;
            }

            while ky as i8 - offset >= 0 {
                let piece = ephemeral_board.get_index(kx, (ky as i8 - offset) as u8).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Rook | Queen)) {
                    return false;
                }

                offset += 1;
            }

            // search up
            let mut offset: i8 = 1;
            // check for king
            if ky < 7 && ephemeral_board.get_index(kx, (ky as i8 + offset) as u8).get_piece() .is_some_and(|piece| piece.r#type == King) {
                return false;
            }

            while ky as i8 + offset <= 7 {
                let piece = ephemeral_board.get_index(kx, (ky as i8 + offset) as u8).get_piece();

                // if a piece of the same color is hit before one of the other color, the king is covered in this direction
                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Rook | Queen)) {
                    return false;
                }

                offset += 1;
            }

            // should probably flip these as "left" is increasing index value from the perspective of the matrix
            // search left
            let mut offset: i8 = 1;
            // check for king
            if kx as i8 - offset >= 0 && ephemeral_board.get_index((kx as i8 - offset) as u8, ky).get_piece().is_some_and(|piece| piece.r#type == King) {
                return false;
            }

            while kx as i8 - offset >= 0 {
                let piece = ephemeral_board.get_index((kx as i8 - offset) as u8, ky).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Rook | Queen)) {
                    return false;
                }

                offset += 1;
            }

            
            // search right
            let mut offset: i8 = 1;
            // check for king
            if kx as i8 + offset <= 7 && ephemeral_board.get_index((kx as i8 + offset) as u8, ky).get_piece().is_some_and(|piece| piece.r#type == King) {
                return false;
            }

            while kx as i8 + offset <= 7 {
                let piece = ephemeral_board.get_index((kx as i8 + offset) as u8, ky).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Rook | Queen)) {
                    return false;
                }

                offset += 1;
            }

            // search down/left
            let mut offset: i8 = 1;
            // check for king or pawn
            if  kx as i8 - offset >= 0 && ky as i8 - offset >= 0 &&
                ephemeral_board
                    .get_index((kx as i8 - offset) as u8, (ky as i8 - offset) as u8)
                    .get_piece()
                    .is_some_and(|piece| 
                        piece.r#type == King || (piece.r#type == Pawn && piece.color == White && self.color == Black) // check if there's a pawn in an attacking position
                    ) // this is almost certainly not the best way to do this
            {
                return false;
            }


            while kx as i8 - offset >= 0 && ky as i8 - offset >= 0 {
                let piece = ephemeral_board.get_index((kx as i8 - offset) as u8, (ky as i8 - offset) as u8).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Bishop | Queen)) {
                    return false;
                }

                offset += 1;
            }
            // move all king/pawn checks to a single area
            // search down/right
            let mut offset: i8 = 1;
            // check for king or pawn
            if kx as i8 + offset <= 7 && ky as i8 - offset >= 0 &&
                ephemeral_board
                    .get_index((kx as i8 + offset) as u8, (ky as i8 - offset) as u8)
                    .get_piece()
                    .is_some_and(|piece| 
                        piece.r#type == King || (piece.r#type == Pawn && piece.color == White && self.color == Black)
                    )
            {
                return false;
            }
            
            while kx as i8 + offset <= 7 && ky as i8 - offset >= 0 {
                let piece = ephemeral_board.get_index((kx as i8 + offset) as u8, (ky as i8 - offset) as u8).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Bishop | Queen)) {
                    return false;
                }

                offset += 1;
            }


            // search up/left
            let mut offset: i8 = 1;
            // check for king or pawn
            if kx as i8 - offset >= 0 && ky as i8 + offset <= 7 &&
                ephemeral_board
                    .get_index((kx as i8 - offset) as u8, (ky as i8 + offset) as u8)
                    .get_piece()
                    .is_some_and(|piece| 
                        piece.r#type == King || (piece.r#type == Pawn && piece.color == White && self.color == Black)
                    )
            {
                return false;
            }
            
            while kx as i8 - offset >= 0 && ky as i8 + offset <= 7 {
                let piece = ephemeral_board.get_index((kx as i8 - offset) as u8, (ky as i8 + offset) as u8).get_piece();

                if piece.is_some_and(|piece| piece.color == self.color) {
                    break;
                }

                if piece.is_some_and(|piece| matches!(piece.r#type, Bishop | Queen)) {
                    return false;
                }

                offset += 1;
            }

            // search up/right
            let mut offset: i8 = 1;
            // check for king or pawn
            if kx as i8 + offset <= 7 && ky as i8 + offset <= 7 &&
                ephemeral_board
                    .get_index((kx as i8 + offset) as u8, (ky as i8 + offset) as u8)
                    .get_piece()
                    .is_some_and(|piece| 
                        piece.r#type == King || (piece.r#type == Pawn && piece.color == White && self.color == Black)
                    )
            {
                return false;
            }
            
            while kx as i8 + offset <= 7 && ky as i8 + offset <= 7 {
                let piece = ephemeral_board.get_index((kx as i8 + offset) as u8, (ky as i8 + offset) as u8).get_piece();
                if piece.is_some_and(|piece| piece.color == self.color) { break }
                if piece.is_some_and(|piece| matches!(piece.r#type, Bishop | Queen)) { return false }

                offset += 1;
            }

            // search for knights
            for (x, y) in Piece::OFFSETS_KNIGHT.iter() {
                if kx as i8 + x > 7 || kx as i8 + x < 0 || ky as i8 + y > 7 || ky as i8 + y < 0 { continue; }
                if ephemeral_board.get_index((kx as i8 + x) as u8, (ky as i8 + y) as u8)
                    .get_piece()
                    .is_some_and(|piece| piece.r#type == Knight && piece.color != self.color)
                {
                    return false;    
                }
            }
            
            return true;
        });

        return offsets.into_iter().map(|(x, y)| (((px as i8 + x) as u8) << 4) + ((py as i8 + y) as u8)).collect();
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