mod board;

use anyhow::Result;

fn main() -> Result<()> {
    let mut b: board::Board = "
        -----
        -----
        -xxx-
        -----
        -----
    "
    .try_into()?;

    for _ in 0..10_000_000 {
        b = b.next();
    }

    Ok(())
}
