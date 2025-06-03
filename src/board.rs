use anyhow::{Error, Result, bail, ensure};

const ALIVE_CHAR: char = 'x';
const DEAD_CHAR: char = '-';

#[derive(Debug, Clone)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn all() -> [Direction; 8] {
        [
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }

    fn offset(&self) -> Offset {
        match self {
            Self::SW => Offset { x: -1, y: -1 },
            Self::S => Offset { x: 0, y: -1 },
            Self::SE => Offset { x: 1, y: -1 },
            Self::W => Offset { x: -1, y: 0 },
            Self::E => Offset { x: 1, y: 0 },
            Self::NW => Offset { x: -1, y: 1 },
            Self::N => Offset { x: 0, y: 1 },
            Self::NE => Offset { x: 1, y: 1 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    dims: Coords,
    cells: Vec<bool>,
}

impl Board {
    pub fn new(dims: Coords, cells: Option<Vec<bool>>) -> Result<Self> {
        let num_cells = dims.x * dims.y;
        ensure!(num_cells > 0, "Board dimensions must be greater than 0");

        let cells = match cells {
            Some(cells) => {
                ensure!(
                    cells.len() == num_cells,
                    "Cells array must match dimensions"
                );
                cells
            }
            None => vec![false; num_cells],
        };

        Ok(Board { cells, dims })
    }

    pub fn dim_y(&self) -> usize {
        self.dims.y
    }

    pub fn dim_x(&self) -> usize {
        self.dims.x
    }

    // Translate 2d coordinates to the flattened vec
    fn index(&self, coords: &Coords) -> usize {
        coords.y * self.dims.x + coords.x
    }

    pub fn alive(&self, coords: &Coords) -> bool {
        self.cells[self.index(coords)]
    }

    fn set_alive(&mut self, coords: &Coords, alive: bool) {
        let idx = self.index(coords);
        self.cells[idx] = alive;
    }

    pub fn add(&self, board: Self, insert_coords: Coords) -> Result<Self> {
        let mut new_board = self.clone();

        if insert_coords.y + board.dim_y() >= self.dim_y()
            || insert_coords.x + board.dim_x() >= self.dim_x()
        {
            bail!("Board must be added inside boundaries");
        }

        for y in 0..board.dim_y() {
            for x in 0..board.dim_x() {
                let new_coords = Coords {
                    x: x + insert_coords.x,
                    y: y + insert_coords.y,
                };
                new_board.set_alive(&new_coords, board.alive(&Coords { x, y }));
            }
        }

        Ok(new_board)
    }

    // Return the coordinates of the neighbor in the specified direction, or None if that would be
    // off the board
    fn neighbor_coords(&self, coords: &Coords, dir: &Direction) -> Option<Coords> {
        let offset = dir.offset();
        let x = coords.x.checked_add_signed(offset.x)?;
        let y = coords.y.checked_add_signed(offset.y)?;

        if x >= self.dims.x || y >= self.dims.y {
            None
        } else {
            Some(Coords { x, y })
        }
    }

    fn neighbor_alive(&self, coords: &Coords, dir: &Direction) -> bool {
        match self.neighbor_coords(coords, dir) {
            Some(neighbor_coords) => self.alive(&neighbor_coords),
            None => false,
        }
    }

    fn num_alive_neighbors(&self, coords: &Coords) -> usize {
        Direction::all()
            .iter()
            .map(|dir| self.neighbor_alive(coords, dir))
            .filter(|&a| a)
            .count()
    }

    fn next_cell_state(&self, coords: &Coords) -> bool {
        match (self.alive(coords), self.num_alive_neighbors(coords)) {
            (true, 0..=1) => false, // Underpopulation
            (true, 2..=3) => true,  // Survival
            (true, 4..) => false,   // Overpopulation
            (false, 3) => true,     // Reproduction
            (false, _) => false,    // Stay dead
        }
    }

    pub fn next(&self) -> Self {
        let mut next_board = Self::new(self.dims.clone(), None).unwrap();

        for y in 0..self.dim_y() {
            for x in 0..self.dim_x() {
                let coords = Coords { x, y };
                next_board.set_alive(&coords, self.next_cell_state(&coords));
            }
        }

        next_board
    }

    pub fn blinker() -> Self {
        "
        -----
        --x--
        --x--
        --x--
        -----
        "
        .try_into()
        .unwrap()
    }

    pub fn gosper() -> Self {
        "
        ---------------------------------------
        --------------------------x------------
        ------------------------x-x------------
        --------------xx------xx------------xx-
        -------------x---x----xx------------xx-
        --xx--------x-----x---xx---------------
        --xx--------x---x-xx----x-x------------
        ------------x-----x-------x------------
        -------------x---x---------------------
        --------------xx-----------------------
        ---------------------------------------
        "
        .try_into()
        .unwrap()
    }
}

// Helper to easily turn human readable strings into a board
impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let s = s.trim();
        if !s
            .chars()
            .all(|c| c.is_whitespace() || c == ALIVE_CHAR || c == DEAD_CHAR)
        {
            bail!("The board string must contain only whitespace, ALIVE_CHAR, or DEAD_CHAR");
        }

        let row_lens: Vec<usize> = s.split_whitespace().map(|row| row.len()).collect();
        if row_lens.iter().skip(1).any(|len| *len != row_lens[0]) {
            bail!("Board rows must all have the same number of columns");
        }
        let dims = Coords {
            x: row_lens[0],
            y: row_lens.len(),
        };

        let cells: Vec<bool> = s
            .chars()
            .filter_map(|c| match c {
                ALIVE_CHAR => Some(true),
                DEAD_CHAR => Some(false),
                _ => None,
            })
            .collect();

        Ok(Board::new(dims, Some(cells))?)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                let coords = Coords { x, y };
                let ch = if self.alive(&coords) {
                    ALIVE_CHAR
                } else {
                    DEAD_CHAR
                };
                write!(f, "{ch}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
