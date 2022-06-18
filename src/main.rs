use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint::Point2;
use ggez::timer::check_update_time;
use ggez::{Context, ContextBuilder, GameResult};

//constants
const BOARD_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    BOARD_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    BOARD_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Snake", "Iamhere345")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("ggez failed to create a context");

    let snake_game = Game::new(&mut ctx);

    event::run(ctx, event_loop, snake_game);
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake_pos: Vec<Pos>,
    snake_dir: Dir,
    controls: Vec<KeyCode>,
}

impl Dir {
    fn from_keycode(state: &Game, key: KeyCode) -> Dir {
        match key {
            KeyCode::Up => Dir::Up,
            KeyCode::Down => Dir::Down,
            KeyCode::Left => Dir::Left,
            KeyCode::Right => Dir::Right,
            _ => state.snake_dir,
        }
    }
}

impl Pos {
    pub fn new() -> Pos {
        Pos { x: 0.0, y: 0.0}
    }
}

impl Game {
    fn new(_ctx: &mut Context) -> Game {
        Game {
            snake_pos: vec![
                Pos {
                    x: 0.0,
                    y: 15.0 * 32.0,
                },
                Pos { x: 0.0, y: -32.0 },
            ],
            snake_dir: Dir::Right,
            controls: vec![KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right],
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 5;

        while check_update_time(ctx, DESIRED_FPS) {
            {
                for key in &self.controls {
                    if keyboard::is_key_pressed(ctx, *key) {
                        self.snake_dir = Dir::from_keycode(self, *key);
                    }
                }
            }

            //snake movement logic
            let mut i: usize = 0;
            let mut old_pos: Pos = Pos::new();

            while i < self.snake_pos.len() {
                if i != 0 {
                    let snake = &self.snake_pos[i - 1];
                    old_pos = *snake;
                }

                {
                    let mut snake_head = &mut self.snake_pos[0];

                    match self.snake_dir {
                        Dir::Up => snake_head.y += 2.0,
                        Dir::Down => snake_head.y -= 2.0,
                        Dir::Right => snake_head.x += 2.0,
                        Dir::Left => snake_head.x -= 2.0,
                    }
                }

                let mut snake_pos = &mut self.snake_pos;

                if i != 0 {
                    snake_pos[i].x = old_pos.x;
                    snake_pos[i].y = old_pos.y;
                }

                i += 1;
            }

            //collision logic
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        for seg_pos in &self.snake_pos {
            println!("{:?}", seg_pos);
            let seg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(seg_pos.x, seg_pos.y, 32.0, 32.0),
                graphics::Color::BLACK,
            )
            .expect("Unable to create mesh.");

            graphics::draw(
                ctx,
                &seg,
                graphics::DrawParam::default().dest(Point2 {
                    x: seg_pos.x,
                    y: seg_pos.y,
                }),
            )
            .expect("Unable to draw mesh.");
        }

        graphics::present(ctx)
    }
}
