use minifb::Error;
use crate::domain::Board;

mod domain;
mod application;

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut board = Board::new(20, 20);
/*    board.alive(2, 10);
    board.alive(3, 10);
    board.alive(4, 10);
*/        board.alive(5, 15);
    board.alive(6, 15);
    board.alive(7, 15);
    board.alive(7, 16);
    board.alive(6, 17);

    println!("cell test is {:?}", board.cell(2, 3));
    println!("cell test is {:?}", board.cell(3, 3));
    println!("cell test is {:?}", board.cell(4, 3));

    board.update();
    println!("cell test is {:?}", board.cell(2, 3));
    println!("cell test is {:?}", board.cell(3, 3));
    println!("cell test is {:?}", board.cell(20, 3));

    application::display(&mut board)?;
    Ok(())
}
