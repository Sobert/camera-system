use ggez::{graphics, Context, ContextBuilder, GameResult, timer};
use ggez::event::{self, EventHandler};
use ggez::graphics::{DrawParam};
use ggez::{nalgebra as na};
use ggez::event::{KeyCode, KeyMods};
use ggez::input::mouse::position;

use rand::Rng;

const NB_OF_POINTS: i32 = 10_000;
const WORLD_WIDTH: f32 = 10_000.0;
const WORLD_HEIGHT: f32 = 10_000.0;
const LOWER_BOUND: Point = Point { x: 10.0, y: 10.0};
const UPPER_BOUND: Point = Point { x: 790.0, y: 590.0};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Camera Demo", "Sobert")
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Point {
    x: f32,
    y: f32,
}

struct MyGame {
    points: Vec<Point>,
    keysdown: Vec<KeyCode>,
    origin: Point,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            points: generate_points(NB_OF_POINTS),
            keysdown: Vec::new(),
            origin: Point {x: 0.0, y: 0.0},
        }
    }
}

fn generate_points(nb: i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    for _ in 0..nb {
        let point = Point {
            x: rng.gen_range(0.0, WORLD_WIDTH),
            y: rng.gen_range(0.0, WORLD_HEIGHT),
        };
        points.push(point);
    }
    points
}

fn draw_point(mb: &mut graphics::MeshBuilder, point: &Point, offset: &Point) {
    mb.line(
        &[
            na::Point2::new(point.x - offset.x, point.y - offset.y ),
            na::Point2::new(point.x - offset.x, point.y + 1.0 - offset.y),
        ],
        1.0,
        graphics::WHITE,
    ).unwrap();
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            for keycode in &self.keysdown {
                if keycode == &KeyCode::Up {
                    self.origin.y = self.origin.y - 2.0;
                }
                if keycode == &KeyCode::Down {
                    self.origin.y = self.origin.y + 2.0;
                }
                if keycode == &KeyCode::Left {
                    self.origin.x = self.origin.x - 2.0;
                }
                if keycode == &KeyCode::Right {
                    self.origin.x = self.origin.x + 2.0;
                }
            }
            let mouse_position = position(ctx);
            println!("{:#?}", mouse_position);

            if mouse_position.x < LOWER_BOUND.x {
                self.origin.x = self.origin.x - 2.0;
            } else if mouse_position.x > UPPER_BOUND.x {
                self.origin.x = self.origin.x + 2.0;
            }
            if mouse_position.y < LOWER_BOUND.y {
                self.origin.y = self.origin.y - 2.0;
            } else if mouse_position.y > UPPER_BOUND.y {
                self.origin.y = self.origin.y + 2.0;
            }

            if self.origin.x < 0.0 {
                self.origin.x = 0.0;
            } else if self.origin.x > WORLD_WIDTH {
                self.origin.x = WORLD_WIDTH
            }
            if self.origin.y < 0.0 {
                self.origin.y = 0.0;
            } else if self.origin.y > WORLD_HEIGHT {
                self.origin.y = WORLD_HEIGHT;
            }
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mb = &mut graphics::MeshBuilder::new();
        for p in &self.points {
            draw_point(mb, p, &self.origin)
        }
        let mesh = mb.build(ctx)?;
        match graphics::draw(ctx, &mesh, DrawParam::new()) {
            Ok(_) => (),
            Err(e) => println!("ERROR : {:#?}", e)
        }
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        self.keysdown.push(keycode);
        self.keysdown.dedup_by_key(|x| *x);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.keysdown.retain(|&x| x != keycode);
    }

}