use anyhow::{bail, ensure, Error, Result};

const ALIVE_CHAR: char = 'x';
const DEAD_CHAR: char = '-';

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }
}

#[derive(Debug)]
struct Board {
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

    fn index(&self, coords: Coords) -> usize {
        coords.y * self.dims.x + coords.x
    }

    pub fn alive(&self, coords: Coords) -> bool {
        self.cells[self.index(coords)]
    }
}

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
                let coords = Coords::new(x, y);
                let ch = if self.alive(coords) {
                    ALIVE_CHAR
                } else {
                    DEAD_CHAR
                };
                write!(f, "{}", ch)?;
            }
            if y < self.dims.y - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let b: Board = "
        x-x
        ---
        x-x
    "
    .try_into()?;

    println!("{b}");
    Ok(())
}
