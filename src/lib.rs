use crate::dynamics::VehicleState;
use nalgebra::Vector2;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::time::Duration;
use tiny_skia::{Color, LineCap, Paint, Path, PathBuilder, Pixmap, Rect, Stroke, Transform};

mod dynamics;
pub mod render;

#[cfg_attr(feature = "python", pyclass)]
#[derive(Default, Debug, Clone)]
pub struct SimState {
    pub vehicle_state: VehicleState,
    pub reward_points: Vec<(Vector2<f32>, f32)>,
}

pub fn get_car_paths(car_pos: Vector2<f32>, car_angle: f32, base_length: f32) -> (Path, Path) {
    let start_vec = Vector2::new(car_pos.x, car_pos.y);
    let length_vec = base_length * Vector2::new(car_angle.cos(), car_angle.sin());
    let end_vec = start_vec + length_vec;
    let half_way_vec = start_vec + length_vec / 2.;

    let mut builder_1 = PathBuilder::new();
    builder_1.move_to(start_vec.x, start_vec.y);
    builder_1.line_to(half_way_vec.x, half_way_vec.y);
    let path1 = builder_1.finish().unwrap();

    let mut builder_2 = PathBuilder::new();
    builder_2.move_to(half_way_vec.x, half_way_vec.y);
    builder_2.line_to(end_vec.x, end_vec.y);
    let path2 = builder_2.finish().unwrap();

    (path1, path2)
}

impl SimState {
    pub fn gen_image_impl(&self) -> Pixmap {
        let mut pixmap = Pixmap::new(1280, 800).unwrap();

        pixmap.fill(Color::from_rgba8(0, 0, 0, 255));

        let mut car_paint = Paint::default();
        car_paint.set_color_rgba8(255, 0, 0, 255);
        let (car_path_1, car_path_2) = get_car_paths(
            Vector2::new(self.vehicle_state.position_x, self.vehicle_state.position_y),
            self.vehicle_state.angle,
            self.vehicle_state.base_length,
        );
        pixmap.stroke_path(
            &car_path_1,
            &car_paint,
            &Stroke {
                width: 50.,
                line_cap: LineCap::Square,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );
        pixmap.stroke_path(
            &car_path_2,
            &car_paint,
            &Stroke {
                width: 50.,
                line_cap: LineCap::Round,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );

        let mut reward_paint = Paint::default();
        reward_paint.set_color_rgba8(0, 255, 0, 255);
        for (pos, _) in &self.reward_points {
            pixmap.fill_rect(
                Rect::from_xywh(pos.x, pos.y, 10., 10.).unwrap(),
                &reward_paint,
                Transform::identity(),
                None,
            );
        }
        pixmap
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl SimState {
    pub fn gen_image_raw(&self) -> Vec<u8> {
        self.gen_image_impl().take()
    }

    pub fn gen_image_png(&self) -> Vec<u8> {
        self.gen_image_impl().encode_png().unwrap()
    }

    pub fn get_vehicle_state_clone(&self) -> VehicleState {
        self.vehicle_state.clone()
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone)]
pub struct Action {
    pub acceleration: f32,
    pub wheel_steer_angle: f32,
}

#[cfg(feature = "python")]
#[pymethods]
impl Action {
    #[new]
    fn new(acceleration: f32, wheel_steer_angle: f32) -> Action {
        Action {
            acceleration,
            wheel_steer_angle,
        }
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone)]
pub struct Simulator {
    state: SimState,
    max_accelerator: f32,
    max_angle: f32,
}

#[cfg(feature = "python")]
#[pymethods]
impl Simulator {
    #[new]
    pub fn new_py(
        initial_vehicle_state: VehicleState,
        reward_points: Vec<(f32, f32, f32)>,
    ) -> Simulator {
        Simulator::new(
            initial_vehicle_state,
            reward_points
                .into_iter()
                .map(|(x, y, r)| (Vector2::new(x, y), r))
                .collect(),
        )
    }

    pub fn advance_s(&mut self, action: Action, delta_t_s: f32) -> f32 {
        self.advance(action, Duration::from_secs_f32(delta_t_s))
    }

    pub fn get_state_clone(&self) -> SimState {
        self.state.clone()
    }
}

impl Simulator {
    pub fn new(
        initial_vehicle_state: VehicleState,
        reward_points: Vec<(Vector2<f32>, f32)>,
    ) -> Simulator {
        Simulator {
            state: SimState {
                vehicle_state: initial_vehicle_state,
                reward_points,
                /*reward_points: vec![
                    // (Vector2::new(10., 10.), 10.),
                    (Vector2::new(1000., 700.), 100.),
                ],*/
            },
            max_accelerator: 100.,
            max_angle: std::f32::consts::FRAC_PI_4,
        }
    }

    pub fn get_state(&self) -> &SimState {
        &self.state
    }

    // This returns a reward
    pub fn advance(&mut self, action: Action, delta_t: Duration) -> f32 {
        let acceleration = action
            .acceleration
            .clamp(-self.max_accelerator, self.max_accelerator);
        let wheel_steer_angle = action
            .wheel_steer_angle
            .clamp(-self.max_angle, self.max_angle);

        self.state
            .vehicle_state
            .step(acceleration, wheel_steer_angle, delta_t);

        let got_reward_fn =
            |val: &(Vector2<f32>, f32)| (self.state.vehicle_state.position() - val.0).norm() < 50.;

        let mut reward = 0.;
        self.state.reward_points.retain(|val| {
            if got_reward_fn(val) {
                reward += val.1;
                false
            } else {
                true
            }
        });

        reward
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn drivingsim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Simulator>()?;
    m.add_class::<Action>()?;
    m.add_class::<dynamics::VehicleState>()?;
    Ok(())
}
