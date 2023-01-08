use board::Board;
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{Event, Style},
};

mod board;

const CELL_SIZE: f32 = 5.;

fn main() {
    let mut window = RenderWindow::new(
        (300 * CELL_SIZE as u32, 200 * CELL_SIZE as u32),
        "GOL",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(60);
    window.set_active(true);
    window.clear(Color::BLACK);

    let mut board = Board::new(300, 300, true);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        let cells = board.get_cells();
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let rect = create_cell((x, y), *cell);

                window.draw(&rect);
            }
        }

        window.display();
        board.next_generation();
    }
}

fn create_cell<'a>(position: (usize, usize), alive: bool) -> RectangleShape<'a> {
    let mut rect = RectangleShape::new();
    rect.set_size(Vector2f::new(CELL_SIZE, CELL_SIZE));
    rect.set_position(Vector2f::new(
        position.0 as f32 * CELL_SIZE,
        position.1 as f32 * CELL_SIZE,
    ));
    if alive {
        rect.set_fill_color(Color::WHITE);
    } else {
        rect.set_fill_color(Color::BLACK);
    }

    return rect;
}
