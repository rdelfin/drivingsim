use crate::dynamics::VehicleState;
use crate::{Action, Simulator};
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra::Vector2;

pub struct GgezRender {
    simulator: Simulator,
    picked_angle: f32,
    w_pressed: bool,
    s_pressed: bool,
}

impl GgezRender {
    pub fn new(_ctx: &mut Context) -> GgezRender {
        GgezRender {
            simulator: Simulator::new(
                VehicleState {
                    position_x: 700.,
                    position_y: 400.,
                    angle: 0.,
                    ..Default::default()
                },
                vec![
                    (Vector2::new(10., 10.), 10.),
                    (Vector2::new(1000., 700.), 100.),
                ],
            ),
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
            50.
        } else {
            -50.
        };
        self.simulator.advance(
            Action {
                acceleration,
                wheel_steer_angle: self.picked_angle * std::f32::consts::PI / 180.,
            },
            delta_t,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 149, 237));

        let state = self.simulator.get_state();
        let car_position = state.vehicle_state.position();
        let angle_display = Text::new(format!("Angle: {}", self.picked_angle));

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(
                    car_position.x,
                    car_position.y - 25.,
                    state.vehicle_state.base_length,
                    50.,
                ))
                .color(Color::RED)
                .rotation(state.vehicle_state.angle),
        );

        for (point, _) in &state.reward_points {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new(point.x - 10., point.y - 10., 20., 20.))
                    .color(Color::GREEN),
            )
        }

        canvas.draw(
            &angle_display,
            graphics::DrawParam::from([10.0, 10.0]).color(Color::WHITE),
        );
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::A => {
                    self.picked_angle -= 10.;
                }
                KeyCode::D => {
                    self.picked_angle += 10.;
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
        .window_mode(WindowMode {
            width: 1280.,
            height: 800.,
            ..Default::default()
        })
        .build()
        .expect("Failed to create game context");
    let game = GgezRender::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
