use crate::grid::{
    CellConstraint, Constraint, CornerConstraint, EdgeConstraint, PuzzleGrid, UniversalPos,
};

/// Puzzle grid implemented using a simple list of constraints by enum
///
/// size:
/// - 32 bytes (the struct itself)
/// - 8*width*height bytes (the contents of grid)
pub struct SingleEnumGrid {
    width: i8,
    height: i8,

    grid: Vec<Constraint>,
}

impl PuzzleGrid for SingleEnumGrid {
    fn new(width: i8, height: i8, pillar: bool) -> Self {
        let universal_width = if pillar {
            2 * (width + 1)
        } else {
            2 * width + 1
        };
        let universal_height = 2 * height + 1;

        let mut grid = vec![];

        for x in 0..universal_width {
            for y in 0..universal_height {
                match (x % 2, y % 2) {
                    (0, 0) => grid.push(Constraint::Corner(CornerConstraint::None)),
                    (1, 1) => grid.push(Constraint::Cell(CellConstraint::None)),
                    _ => grid.push(Constraint::Edge(EdgeConstraint::None)),
                }
            }
        }

        Self {
            width: universal_width,
            height: universal_height,

            grid,
        }
    }

    fn universal_width(&self) -> i8 {
        self.width
    }

    fn universal_height(&self) -> i8 {
        self.height
    }

    #[inline(always)]
    fn get_corner(&self, coords: &super::UniversalPos) -> super::CornerConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(x % 2, 0, "get_corner must only be called with even coords!");
        assert_eq!(y % 2, 0, "get_corner must only be called with even coords!");
        match self.get_any(coords) {
            Constraint::Corner(corner_constraint) => corner_constraint,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn get_edge(&self, coords: &super::UniversalPos) -> super::EdgeConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(
            (x + y) % 2,
            1,
            "get_edge must only be called with exactly one odd coord!"
        );
        match self.get_any(coords) {
            Constraint::Edge(edge_constraint) => edge_constraint,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn get_cell(&self, coords: &super::UniversalPos) -> super::CellConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(x % 2, 1, "get_cell must only be called with odd coords!");
        assert_eq!(y % 2, 1, "get_cell must only be called with odd coords!");
        match self.get_any(coords) {
            Constraint::Cell(cell_constraint) => cell_constraint,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn get_any(&self, coords: &UniversalPos) -> Constraint {
        self.grid[(coords.y * self.width + coords.x) as usize]
    }
}

union UnionConstraint {
    corner: CornerConstraint,
    edge: EdgeConstraint,
    cell: CellConstraint,
}

impl UnionConstraint {
    pub fn new_corner(corner: CornerConstraint) -> Self {
        Self { corner }
    }
    pub fn new_cell(cell: CellConstraint) -> Self {
        Self { cell }
    }
    pub fn new_edge(edge: EdgeConstraint) -> Self {
        Self { edge }
    }
}

/// Puzzle grid implemented using a simple list of constraints by union
///
/// size:
/// - 32 bytes (the struct itself)
/// - 8*width*height bytes (the contents of grid)
pub struct SingleUnionGrid {
    width: i8,
    height: i8,

    grid: Vec<UnionConstraint>,
}

impl PuzzleGrid for SingleUnionGrid {
    fn new(width: i8, height: i8, pillar: bool) -> Self {
        let universal_width = if pillar {
            2 * (width + 1)
        } else {
            2 * width + 1
        };
        let universal_height = 2 * height + 1;

        let mut grid = vec![];
        for x in 0..universal_width {
            for y in 0..universal_height {
                match (x % 2, y % 2) {
                    (0, 0) => grid.push(UnionConstraint::new_corner(CornerConstraint::None)),
                    (1, 1) => grid.push(UnionConstraint::new_cell(CellConstraint::None)),
                    _ => grid.push(UnionConstraint::new_edge(EdgeConstraint::None)),
                }
            }
        }

        Self {
            width: universal_width,
            height: universal_height,

            grid,
        }
    }

    fn universal_width(&self) -> i8 {
        self.width
    }

    fn universal_height(&self) -> i8 {
        self.height
    }

    #[inline(always)]
    fn get_corner(&self, coords: &super::UniversalPos) -> super::CornerConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(x % 2, 0, "get_corner must only be called with even coords!");
        assert_eq!(y % 2, 0, "get_corner must only be called with even coords!");
        unsafe { self.grid[(y * self.width + x) as usize].corner }
    }

    #[inline(always)]
    fn get_edge(&self, coords: &super::UniversalPos) -> super::EdgeConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(
            (x + y) % 2,
            1,
            "get_edge must only be called with exactly one odd coord!"
        );
        unsafe { self.grid[(y * self.width + x) as usize].edge }
    }

    #[inline(always)]
    fn get_cell(&self, coords: &super::UniversalPos) -> super::CellConstraint {
        let UniversalPos { x, y } = coords;
        assert_eq!(x % 2, 1, "get_cell must only be called with odd coords!");
        assert_eq!(y % 2, 1, "get_cell must only be called with odd coords!");
        unsafe { self.grid[(y * self.width + x) as usize].cell }
    }
}
