use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

#[macroquad::main("Game Of Life")]
async fn main() {
    const WIDTH: usize = 640; // pixels
    const HEIGHT: usize = 480; // pixels
    const CELL_SIZE: usize = 8; // pixels
    const NUM_COLUMNS: usize = WIDTH / CELL_SIZE;
    const NUM_ROWS: usize = HEIGHT / CELL_SIZE;
    const NUM_STARTING_LIVING_CELLS: i32 = 300;
    let mut board: [[CellState; NUM_COLUMNS]; NUM_ROWS] =
        [[CellState::Dead; NUM_COLUMNS]; NUM_ROWS];
    let mut temp_board: [[CellState; NUM_COLUMNS]; NUM_ROWS] =
        [[CellState::Dead; NUM_COLUMNS]; NUM_ROWS];

    request_new_screen_size(WIDTH as f32, HEIGHT as f32);

    // populate initial living cells
    for _ in 0..NUM_STARTING_LIVING_CELLS {
        let rand_x = rand::gen_range(0, NUM_COLUMNS);
        let rand_y = rand::gen_range(0, NUM_ROWS);
        board[rand_y][rand_x] = CellState::Alive;
    }

    loop {
        clear_background(RED);

        // update board
        for (y, row) in board.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                let mut living_neighbors = 0;
                for i in -1i32..=1 {
                    for j in -1i32..=1 {
                        let new_y = y as i32 + i;
                        let new_x = x as i32 + j;
                        if new_y < 0
                            || new_y >= NUM_ROWS as i32
                            || new_x < 0
                            || new_x >= NUM_COLUMNS as i32
                        {
                            continue;
                        }
                        // ignore current cell
                        if i == 0 && j == 0 {
                            continue;
                        }
                        if board[new_y as usize][new_x as usize] == CellState::Alive {
                            living_neighbors += 1;
                        }
                    }
                }
                temp_board[y][x] = match (cell, living_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                    // Rule 3: Any live cell with more than three live neighbors dies, as if by overpopulation.
                    (CellState::Alive, x) if x < 2 || x > 3 => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbors lives on to the next generation.
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 4: Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => CellState::Alive,
                    (_, _) => CellState::Dead,
                }
            }
        }

        board = temp_board;

        // draw board
        for (y, row) in board.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                let color: Color;
                match cell {
                    CellState::Dead => {
                        color = BLACK;
                    }
                    CellState::Alive => {
                        color = WHITE;
                    }
                }

                draw_rectangle(
                    x as f32 * CELL_SIZE as f32,
                    y as f32 * CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    color,
                );
            }
        }

        next_frame().await
    }
}
