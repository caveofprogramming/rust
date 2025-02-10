use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_STARS: usize = 1000;

fn main() {
    let mut window = Window::new(
        "Starfield - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rng = rand::rng();

    for _ in 0..NUM_STARS {
        let x = rng.random_range(0..WIDTH);
        let y = rng.random_range(0..HEIGHT);
        let color = rng.random_range(0..0xFFFFFF);
        buffer[y * WIDTH + x] = color;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
