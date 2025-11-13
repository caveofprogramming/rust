use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const STAR_COUNT: usize = 100;

struct Star {
    x: f32,
    y: f32,
    speed: f32,
}

impl Star {
    fn new() -> Self {
        let mut rng = rand::rng();
        Star {
            x: rng.random_range(0.0..WIDTH as f32),
            y: rng.random_range(0.0..HEIGHT as f32),
            speed: rng.random_range(0.5..2.0),
        }
    }

    fn update(&mut self) {
        self.y += self.speed;
        if self.y > HEIGHT as f32 {
            self.y = 0.0;
            self.x = rand::rng().random_range(0.0..WIDTH as f32);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Starfield",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut stars: Vec<Star> = (0..STAR_COUNT).map(|_| Star::new()).collect();

    let mut rng = rand::rng();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for star in &mut stars {
            star.update();
        }

        buffer.fill(0);
        for star in &stars {
            let x = star.x as usize;
            let y = star.y as usize;
            let idx = y * WIDTH + x;
            if idx < buffer.len() {
                let red:i32 = rng.random_range(0 .. 0xFF as i32);
                let green:i32 = rng.random_range(0 .. 0xFF as i32);
                let blue:i32 = rng.random_range(0 .. 0xFF as i32);
                let color:i32 = (red << 16) + (green << 8) + blue;

                buffer[idx] = color.try_into().unwrap();
                
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
