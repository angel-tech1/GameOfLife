use std::io::Write;

fn main() {
    let mut grid = [[false; 100]; 100];

    // Adding some random cells
    grid[60][41] = true;
    grid[60][40] = true;
    grid[61][41] = true;
    grid[59][42] = true;
    grid[62][42] = true;
    grid[68][30] = true;
    grid[68][31] = true;
    grid[61][51] = true;
    grid[19][62] = true;
    grid[32][02] = true;

    loop {
        // Print the grid
        for row in grid {
            for cell in row {
                if cell {
                    print!("😶");
                } else {
                    print!("・");
                }
            }
            println!();
        }

        // Calculate the next generation of the grid
        for i in 0..100 {
            for j in 0..100 {
                let alive_neighbors = count_alive_neighbors(&grid, i, j);

                // Apply the Game of Life rules
                if grid[i][j] {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        grid[i][j] = false;
                    }
                } else {
                    if alive_neighbors == 3 {
                        grid[i][j] = true;
                    }
                }
            }
        }

        // Wait for user input
        println!("Press Enter to continue...");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

fn count_alive_neighbors(grid: &[[bool; 100]], i: usize, j: usize) -> u8 {
    let mut alive_neighbors = 0;

    let left_limit = -1_isize;

    // Count the number of alive neighbors
    for x_offset in left_limit..2 {
        for y_offset in left_limit..2 {
            if x_offset == 0 && y_offset == 0 {
                continue; // the current position doesn't count as a neighbor
            }

            let i2 = add_offset(i, x_offset);

            let j2 = add_offset(j, y_offset);

            if i2 >= 100 || j2 >= 100 {
                continue; // out of boundaries
            }

            let alive = if grid[i2][j2] { 1 } else { 0 };
            alive_neighbors += alive;
        }
    }

    return alive_neighbors;
}

fn add_offset(index: usize, offset: isize) -> usize {
    return if offset.is_negative() && index == 0 {
        0
    } else if offset.is_negative() {
        index - offset.unsigned_abs()
    } else {
        index + offset.unsigned_abs()
    }
}
