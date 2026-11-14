use std::collections::HashMap;

use colored::Colorize as _;

use crate::puzzle::*;

const DR: &str = "┌";
const DL: &str = "┐";
const UR: &str = "└";
const UL: &str = "┘";
const LR: &str = "───";
const UD: &str = "│";

const DLR: &str = "┬";
const ULR: &str = "┴";
const UDR: &str = "├";
const UDL: &str = "┤";

const UDLR: &str = "┼";

const BROKEN_LR: &str = "─ ─";
const BROKEN_UD: &str = " ";

impl Poly {
    const MAX_INLINE_WIDTH: usize = 6;
    const MAX_INLINE_HEIGHT: usize = 4;

    fn can_display_inline(&self) -> bool {
        let (width, height) = self.dims();

        !self.rotatable()
            && width as usize <= Self::MAX_INLINE_WIDTH
            && height as usize <= Self::MAX_INLINE_HEIGHT
    }

    /// Return a braille representation of the poly
    fn display_inline(&self, center: bool) -> String {
        let mut disp_poly = self.clone();
        disp_poly.normalize();

        // Create bitmap
        let mut bitmap = [[false; Self::MAX_INLINE_WIDTH]; Self::MAX_INLINE_HEIGHT];
        for block in disp_poly.minos() {
            bitmap[block.y as usize][block.x as usize] = true
        }

        // Center bitmap so it looks not stupid
        if center {
            let (width, _) = disp_poly.dims();

            // Shift everything by half of the empty space to make it center-ish
            let shift_amount = (Self::MAX_INLINE_WIDTH - width as usize) / 2;
            for line in &mut bitmap {
                for x in (shift_amount..Self::MAX_INLINE_WIDTH).rev() {
                    line[x] = line[x - shift_amount];
                    line[x - shift_amount] = false;
                }
            }
        }

        // Transform bitmap into braille
        let mut res = String::new();

        for char_id in 0..3 {
            let mut char = 0x2800_u32;

            let offset = char_id * 2;

            if bitmap[3][offset] {
                char += 0x01;
            }
            if bitmap[2][offset] {
                char += 0x02;
            }
            if bitmap[1][offset] {
                char += 0x04;
            }
            if bitmap[0][offset] {
                char += 0x40;
            }

            if bitmap[3][offset + 1] {
                char += 0x08;
            }
            if bitmap[2][offset + 1] {
                char += 0x10;
            }
            if bitmap[1][offset + 1] {
                char += 0x20;
            }
            if bitmap[0][offset + 1] {
                char += 0x80;
            }

            res.push(char::from_u32(char).unwrap());
        }

        res
    }

    /// Return a full representation of the poly
    fn display_legend(&self) -> String {
        let (width, height) = self.dims();

        let mut res = String::new();

        for y in (0..height).rev() {
            for x in 0..width {
                res.push_str(&if self.minos().contains(&Pos::new(x, y)) {
                    map_on_color("  ", ORANGE_ID)
                } else {
                    "  ".into()
                })
            }
            res.push('\n');
        }

        res
    }
}

impl Puzzle {
    fn display_line(&self, solution: Option<&SolutionPath>, y: i8) -> String {
        let mut res = String::new();

        // First column
        res.push_str(&color_vertex(
            self,
            solution,
            &Pos::new(0, y),
            match y {
                0 => UR,                               // Bottom left corner
                height if height == self.height => DR, // Left side
                _ => UDR,                              // Top left corner
            },
        ));
        for x in 0..self.width {
            if x != 0 {
                // Middle columns
                res.push_str(&color_vertex(
                    self,
                    solution,
                    &Pos::new(x, y),
                    match y {
                        0 => ULR,                               // Bottom vertex
                        height if height == self.height => DLR, // Middle vertex
                        _ => UDLR,                              // Top vertex
                    },
                ));
            }

            let edge = EdgePos::new(x, y, Direction::Right);
            if self.is_blocked(&edge) {
                res.push_str(BROKEN_LR);
            } else {
                // Color edge red if it has a stone
                res.push_str(&color_edge(self, solution, &edge, LR));
            }
        }
        // Last column
        res.push_str(&color_vertex(
            self,
            solution,
            &Pos::new(self.width, y),
            match y {
                0 => UL,                               // Bottom right corner
                height if height == self.height => DL, // Right side
                _ => UDL,                              // Top right corner
            },
        ));

        res
    }

    /// Generate a unicode graphical representation of the puzzle, with the given solution path overlaid on top
    pub fn display_with_solution(&self, solution: Option<&SolutionPath>) -> String {
        let mut current_legend: u8 = b'A';
        // Maps the polyomimo representation to its legend
        let mut polyomino_legend: HashMap<Poly, String> = HashMap::new();

        let mut res = String::new();

        res.push_str(&self.display_line(solution, self.height));
        res.push('\n');

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let edge = EdgePos::new(x, y, Direction::Up);
                if self.is_blocked(&edge) {
                    res.push_str(BROKEN_UD);
                } else {
                    res.push_str(&color_edge(self, solution, &edge, UD));
                }

                match self.get_cell_content(&Pos::new(x, y)) {
                    CellType::Empty => res.push_str("   "),
                    CellType::Square(c) => res.push_str(&map_color(" ■ ", c)),
                    CellType::Star(c) => res.push_str(&map_color(" ✹ ", c)),
                    CellType::Triangle(n) => res.push_str(&map_color(&format!(" {n} "), ORANGE_ID)),
                    CellType::Poly(p) => {
                        if p.can_display_inline() {
                            res.push_str(&p.display_inline(true))
                        } else {
                            let legend = polyomino_legend.entry(p.clone()).or_insert_with(|| {
                                let legend = String::from_utf8_lossy_owned(vec![current_legend]);
                                current_legend += 1;
                                legend
                            });

                            res.push_str(&format!(" {legend} "));
                        }
                    }
                    CellType::Ylop(p) => {
                        if p.can_display_inline() {
                            res.push_str(
                                &p.display_inline(true)
                                    .on_color(colored::Color::BrightCyan)
                                    .to_string(),
                            )
                        } else {
                            let legend = polyomino_legend
                                .entry(p.clone())
                                .or_insert_with(|| {
                                    let legend =
                                        String::from_utf8_lossy_owned(vec![current_legend]);
                                    current_legend += 1;
                                    legend
                                })
                                .on_color(colored::Color::BrightCyan)
                                .to_string();

                            res.push_str(&format!(" {legend} "));
                        }
                    }
                    CellType::Canceller(c) => res.push_str(&map_color(" 🟀 ", c)),
                }
            }

            let edge = EdgePos::new(self.width, y, Direction::Up);
            if self.is_blocked(&edge) {
                res.push_str(BROKEN_UD);
            } else {
                res.push_str(&color_edge(self, solution, &edge, UD));
            }

            res.push('\n');
            res.push_str(&self.display_line(solution, y));
            res.push('\n');
        }

        // Generate legend for too-large polys
        for (poly, legend) in polyomino_legend {
            let rotatable = if poly.rotatable() {
                "rotatable"
            } else {
                ""
            };

            res.push_str(&format!("{}: {}\n", legend, rotatable));
            res.push_str(&poly.display_legend());
        }

        res
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_with_solution(None))
    }
}

// Meta colors
const START_COLOR: colored::Color = colored::Color::Green;
const END_COLOR: colored::Color = colored::Color::Blue;
const SOLUTION_COLOR: colored::Color = colored::Color::BrightBlack;

// Custom colors
const STONE_COLOR: colored::Color = colored::Color::Red;
const ORANGE_ID: u8 = 200;

// Coloring utils
fn map_color(s: &str, color_id: u8) -> String {
    let res = match color_id {
        0 => s.color(colored::Color::Red),
        1 => s.color(colored::Color::Magenta),
        2 => s.color(colored::Color::Green),
        3 => s.color(colored::Color::Yellow),
        4 => s.color(colored::Color::Blue),
        5 => s.color(colored::Color::Cyan),
        ORANGE_ID => s.custom_color(colored::customcolors::CustomColor::new(255, 150, 80)),
        x => panic!("Unsupported color: {x}"),
    };

    res.to_string()
}

fn map_on_color(s: &str, color_id: u8) -> String {
    let res = match color_id {
        0 => s.on_color(colored::Color::Red),
        1 => s.on_color(colored::Color::Magenta),
        2 => s.on_color(colored::Color::Green),
        3 => s.on_color(colored::Color::Yellow),
        4 => s.on_color(colored::Color::Blue),
        5 => s.on_color(colored::Color::Cyan),
        ORANGE_ID => s.on_custom_color(colored::customcolors::CustomColor::new(255, 150, 80)),
        x => panic!("Unsupported color: {x}"),
    };

    res.to_string()
}

fn color_edge(p: &Puzzle, solution: Option<&SolutionPath>, edge: &EdgePos, s: &str) -> String {
    let bg = if let Some(sol) = solution
        && sol.try_as_edge_path().unwrap().contains(edge)
    {
        s.on_color(SOLUTION_COLOR).to_string()
    } else {
        s.to_string()
    };

    if p.edge_stones.contains(edge) {
        bg.color(STONE_COLOR).to_string()
    } else {
        bg
    }
}

fn color_vertex(p: &Puzzle, solution: Option<&SolutionPath>, pos: &Pos, s: &str) -> String {
    let bg = if p.starts.contains(pos) {
        s.on_color(START_COLOR).to_string()
    } else if p.ends.contains(pos) {
        s.on_color(END_COLOR).to_string()
    } else if let Some(sol) = solution
        && sol.contains(pos)
    {
        s.on_color(SOLUTION_COLOR).to_string()
    } else {
        s.to_string()
    };

    if p.vertex_stones.contains(pos) {
        bg.color(STONE_COLOR).to_string()
    } else {
        bg
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::*;

    #[test]
    fn test_poly_inline() {
        let poly = Poly::new(
            false,
            vec![
                Pos::new(3, 0),
                Pos::new(3, 1),
                Pos::new(3, 2),
                Pos::new(4, 2),
            ],
        );

        // The characters in these assers are not spaces, they are braille patters with no dots
        assert_eq!("⡖⠀⠀", poly.display_inline(false));
        assert_eq!("⠀⡖⠀", poly.display_inline(true));
    }
}
