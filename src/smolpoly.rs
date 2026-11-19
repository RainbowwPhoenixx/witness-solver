//! Implements the logic for bitboard polys
//! 
//! TODO:
//! - make a type for and 8x8 bitboards and implement all the bit
//!   shift methods on it for better organization

use crate::puzzle::Pos;

/// Polymino implementation using bitmasks
///
/// These polys have a maximum size of 8x8
/// A layout of a poly is this:
///
/// ```text
/// y7 | 57 58 59 60 61 62 63 64
/// y6 | 49 50 51 52 53 54 55 56
/// y5 | 41 42 43 44 45 46 47 48
/// y4 | 33 34 35 36 37 38 39 40
/// y3 | 25 26 27 28 29 30 31 32
/// y2 | 17 18 19 20 21 22 23 24
/// y1 |  9 10 11 12 13 14 15 16
/// y0 |  1  2  3  4  5  6  7  8
///     ------------------------
///      x0 x1 x2 x3 x4 x5 x6 x7
/// ```
/// 
/// TODO: would storing the width/height of the mino improve performance?
#[derive(Clone, Debug)]
pub struct SmolPoly {
    /// List of all the rotations of the mino
    ///
    /// A mino of 0 is used to indicate that there are no more rotations
    /// which can happen in one of three cases:
    /// - The mino cannot rotate, in which case minos[1] will be 0
    /// - The mino can rotate, but has 90° rotational symmetry and
    ///   is therefore the same case as above
    /// - The mino has 180° rotational symmetry, in which case minos[2] will be 0
    ///
    /// This is not a Vec<u64> because a vec with one element in it is
    /// a pointer + size + capacity + the element, which is 4*u64.
    rotations: [u64; 4],
}

impl From<u32> for SmolPoly {
    fn from(value: u32) -> Self {
        let rotatable = (value & 1 << 20) != 0;

        let mut mino = 0;

        // translate from the game's coord system to ours
        for x in 0..4 {
            for y in 0..4 {
                if (value & (1 << (x * 4 + 3 - y))) != 0 {
                    mino |= 1 << (x + y * 8);
                }
            }
        }

        SmolPoly::new_from_bitmask(mino, rotatable)
    }
}

impl SmolPoly {
    /// Mino mask where the left column is all ones
    const LEFT_COLUMN: u64 = 0x0101010101010101;
    /// Mino mask where the bottom row is all
    const BOTTOM_ROW: u64 = 0xff;

    /// Ignores coords outside of [0,7]
    pub fn from_coords(coords: Vec<Pos>, rotatable: bool) -> Self {
        let mut mino = 0;

        for Pos { x, y } in coords {
            mino |= 1 << (x + y * 8);
        }

        Self::new_from_bitmask(mino, rotatable)
    }

    /// Returns the number of minos in the poly
    #[inline(always)]
    pub fn mino_count(&self) -> u32 {
        self.rotations[0].count_ones()
    }

    /// Returns the bitboards of the rotated polys
    #[inline(always)]
    pub fn get_rotations(&self) -> &[u64; 4] {
        &self.rotations
    }

    /// Shifts the poly such that the bottom left is at the given coords
    /// 
    /// This is then no longer a valid `SmolPoly` as it is denormalized
    #[inline(always)]
    pub fn shift(poly: u64, x: u8, y: u8) -> u64 {
        poly << x + y * 8
    }

    /// Create a mino from the "initial" rotation
    pub fn new_from_bitmask(mino: u64, rotatable: bool) -> Self {
        let mut minos = [0u64; 4];
        minos[0] = Self::normalise(mino);

        if rotatable {
            // rotate 90 deg
            let mut r90 = 0;

            // this is terribly slow for what it is, but this is only ran once when the
            // piece is initialized, so who cares
            for x in 0..8 {
                for y in 0..8 {
                    if mino & 1 << (x + y * 8) != 0 {
                        r90 |= 1 << (y  + (7 - x) * 8)
                    }
                }
            }
            r90 = Self::normalise(r90);

            if minos[0] == r90 {
                return Self { rotations: minos };
            }
            minos[1] = r90;

            // rotate 180 deg (equivalent to reversing bits)
            let r180 = Self::normalise(mino.reverse_bits());
            // if same as original, stop and return
            if minos[0] == r180 {
                return Self { rotations: minos };
            }
            minos[2] = r180;

            // rotate -90 deg (-90 is 90 + 180)
            minos[3] = Self::normalise(minos[1].reverse_bits())
        }

        Self { rotations: minos }
    }

    /// Shifts the mino down and to the left as much as possible
    ///
    /// # Panics
    ///
    /// Panics if the mino is 0
    fn normalise(mut mino: u64) -> u64 {
        if mino == 0 {
            panic!("Cannot normalise a 0 mino");
        }

        // Make mino go down while bottom row is empty
        while mino & Self::BOTTOM_ROW == 0 {
            mino >>= 8
        }

        // Make mino go left while left column is empty
        while mino & Self::LEFT_COLUMN == 0 {
            mino >>= 1
        }

        mino
    }
}

impl std::fmt::Display for SmolPoly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rot in self.rotations {
            if rot != 0 {
                write!(f, "{:08b}\n", (((rot >> 56) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 48) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 40) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 32) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 24) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 16) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n", (((rot >> 8) & 0xff) as u8).reverse_bits())?;
                write!(f, "{:08b}\n\n", (rot as u8).reverse_bits())?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
