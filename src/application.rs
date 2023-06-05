use minifb::{Key, Window, WindowOptions, Result};
use minifb::Key::C;
use crate::domain::{Board, State};

const CELL_SIZE: usize = 20;

pub fn display(board: &mut Board) -> Result<()> {
    let mut buffer: Vec<u32> = vec![0; board.width() * board.height()];

    let windows_width: usize = board.width() * CELL_SIZE;
    let windows_height: usize = board.height() * CELL_SIZE;

    let mut window = Window::new(
        "Test - ESC to exit",
        windows_width,
        windows_height,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(200)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("refresh");
        board.update();
        refresh_buffer(board, &mut buffer);
        println!("cell test is {:?}", board.cell(2, 3));
        println!("cell test is {:?}", board.cell(3, 3));
        println!("cell test is {:?}", board.cell(4, 3));


        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, board.width(), board.height())?;
    }
    Ok(())
}

fn refresh_buffer(board: &mut Board, buffer: &mut Vec<u32>) {
    for i in 0..board.width() * board.height()  {
        let x = i % board.width();
        let y = i / board.width();
        let state = board.cell((x) as isize, (y) as isize);
        let color = match state {
            State::Dead => 0,
            State::Alive => u32::MAX,
        };
        buffer.insert(i, color);
    }
}
