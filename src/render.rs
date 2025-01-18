use crate::{Action, Simulator};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};

pub struct GgezRender {
    simulator: Simulator,
    picked_angle: f32,
    w_pressed: bool,
    s_pressed: bool,
}

impl GgezRender {
    pub fn new(_ctx: &mut Context) -> GgezRender {
        GgezRender {
            simulator: Simulator::new(),
            picked_angle: 0.,
            w_pressed: false,
            s_pressed: false,
        }
    }
}

impl EventHandler for GgezRender {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_t = ctx.time.delta();
        let acceleration = if self.w_pressed == self.s_pressed {
            0.
        } else if self.w_pressed {
            10.
        } else {
            -10.
        };
        self.simulator.advance(
            Action {
                acceleration,
                wheel_steer_angle: self.picked_angle,
            },
            delta_t,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let state = self.simulator.get_state();
        let car_position = state.vehicle_state.position;

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 149, 237));
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    car_position.x - 2.,
                    car_position.y - state.vehicle_state.base_length / 2.,
                    4.,
                    state.vehicle_state.base_length,
                ))
                .color(Color::RED)
                .rotation(state.vehicle_state.angle),
        );
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::A => {
                    self.picked_angle += 5.;
                }
                KeyCode::D => {
                    self.picked_angle -= 5.;
                }
                KeyCode::W => {
                    self.w_pressed = true;
                }
                KeyCode::S => {
                    self.s_pressed = true;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::W => {
                    self.w_pressed = false;
                }
                KeyCode::S => {
                    self.s_pressed = false;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub fn run() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("vehicle_sim", "Ricardo Delfin")
        .build()
        .expect("Failed to create game context");
    let game = GgezRender::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
