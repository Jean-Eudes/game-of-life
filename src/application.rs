use minifb::{Key, Result, Window, WindowOptions};

use crate::domain::{Board, State};

const WIDTH_SCREEN_SIZE: usize = 1000;
const HEIGHT_SCREEN_SIZE: usize = 1000;

const CELL_SIZE: usize = 10;
const WIDTH: usize = WIDTH_SCREEN_SIZE / CELL_SIZE;
const HEIGHT: usize = HEIGHT_SCREEN_SIZE / CELL_SIZE;

pub fn display() -> Result<()> {
    let mut board = Board::new(WIDTH, HEIGHT, vec!((5, 45), (6, 45), (7, 45), (7, 46), (6, 47)));
    let mut buffer = vec![0; board.width() * board.height()];

    let windows_width: usize = board.width() * CELL_SIZE;
    let windows_height: usize = board.height() * CELL_SIZE;

    let mut window = Window::new(
        "Game of life - ESC to exit",
        windows_width,
        windows_height,
        WindowOptions::default(),
    )?;

     window.set_target_fps(12);
    // window.limit_update_rate(Some(std::time::Duration::from_millis(200)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::V) {
            board = Board::new(WIDTH, HEIGHT, vec!((5, 45), (6, 45), (7, 45), (7, 46), (6, 47)));
        } else if window.is_key_down(Key::B) {
            board = Board::new(WIDTH, HEIGHT, vec!((2, 3), (3, 3), (4, 3)));
        } else if window.is_key_down(Key::G) {
            board = Board::new(WIDTH, HEIGHT, vec!((2, 10), (3, 10), (4, 10), (3, 11), (4, 11), (5, 11)));
        } else if window.is_key_down(Key::F) {
            board = Board::new(WIDTH, HEIGHT, vec!((20, 22), (21, 22), (22, 22), (19, 23), (21, 23), (23, 23), (20, 24), (21, 24), (22, 24)));
        }
        board.update();
        refresh_buffer(&mut board, &mut buffer);
        window.update_with_buffer(&buffer, board.width(), board.height())?;
    }
    Ok(())
}

fn refresh_buffer(board: &mut Board, buffer: &mut Vec<u32>) {
    for i in 0..board.width() * board.height() {
        let (x, y) = board.coordinates(i);
        let state = board.cell(x, y);
        let color = match state {
            State::Dead => 0x000000,
            State::Alive => 0xff0000,
        };
        buffer[i] = color;
    }
}
