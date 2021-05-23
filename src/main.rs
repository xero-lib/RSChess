mod pieces;

use std::fmt::{self, Display};

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::MaybeUninit::uninit().assume_init();
        for (i, place) in items.iter_mut().enumerate() {
            std::ptr::write(place, $constructor(i));
        }
        items
    }}
}

struct Board([Option<Box<dyn pieces::Piece>>; 8*8]);

fn main() {
    let board = Board(unsafe { make_array!(8*8, |_| None)});
    println!("{}", &board);
}

impl Board {
    pub fn is_white(x: usize, y: usize) -> bool {
        (x ^ y) & 1  == 0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "{} ", match Board::is_white(x, y) {
                    true => ' ',
                    false => '#',
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//light and dark squared board //? colorboard
//data board    //? compboard
//pieces
//piece offsets
//piece offset calculator
//offset to location
//turn counter
//move pieces


/*
king: [
		[-1, -1],
		[0, -1],
		[1, -1],
		[-1, 0],
		[1, 0],
		[-1, 1],
		[0, 1],
		[1, 1],
	],
	rook: new Array(32), //assign after
	queen: [], //assign after
	bishop: new Array(32), //assign after
	knight: [
		[-1, -2],
		[1, -2],
		[-2, -1],
		[2, -1],
		[-2, 1],
		[2, 1],
		[-1, 2],
		[1, 2],
	],
	pawn: {
		light: {
			first: [2, 0],
			move: [1, 0],
			capture: [
				[1, -1],
				[1, 1],
			],
		},
		dark: {
			first: [-2, 0],
			move: [-1, 0],
			capture: [
				[-1, -1],
				[-1, 1],
			],
		},
	},
};

//assign final rook offsets
for (let i = 0; i < 8; i++) {
	offsets.rook[i] = [i + 1, 0];
	offsets.rook[i + 8] = [0, i + 1];
	offsets.rook[i + 16] = [0 - (i + 1), 0];
	offsets.rook[i + 24] = [0, 0 - (i + 1)];
}
//assign final bishop offsets
for (let i = 0; i < 8; i++) {
	offsets.bishop[i] = [i + 1, i + 1];
	offsets.bishop[i + 8] = [0 - (i + 1), 0 - (i + 1)];
	offsets.bishop[i + 16] = [i + 1, 0 - (i + 1)];
	offsets.bishop[i + 24] = [0 - (i + 1), i + 1];
}

//assign final queen offsets
offsets.queen.push(...offsets.rook, ...offsets.bishop);

export default offsets;

*/