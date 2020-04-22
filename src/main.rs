use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::screen_coordinates;
use ggez::graphics::{DrawParam};
use ggez::{nalgebra as na};

use rand::Rng;

const NB_OF_POINTS: i32 = 50;

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
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            points: generate_points(ctx, NB_OF_POINTS),
        }
    }
}

fn generate_points(ctx: &Context, nb: i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    let screen = screen_coordinates(ctx);
    for _ in 0..nb {
        let point = Point {
            x: rng.gen_range(0.0, screen.w),
            y: rng.gen_range(0.0, screen.h),
        };
        points.push(point);
    }
    points
}

fn draw_point(mb: &mut graphics::MeshBuilder, point: &Point) {
    mb.line(
        &[
            na::Point2::new(point.x, point.y ),
            na::Point2::new(point.x, point.y + 1.0),
        ],
        1.0,
        graphics::WHITE,
    ).unwrap();
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mb = &mut graphics::MeshBuilder::new();
        for p in &self.points {
            draw_point(mb, p)
        }
        let mesh = mb.build(ctx)?;
        match graphics::draw(ctx, &mesh, DrawParam::new()) {
            Ok(_) => (),
            Err(e) => println!("ERROR : {:#?}", e)
        }
        graphics::present(ctx)
    }
}