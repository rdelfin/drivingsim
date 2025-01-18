use crate::dynamics::VehicleState;
use nalgebra::Vector2;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::time::Duration;

mod dynamics;
pub mod render;

#[cfg_attr(feature = "python", pyclass)]
#[derive(Default, Debug, Clone)]
pub struct SimState {
    pub vehicle_state: VehicleState,
    pub reward_points: Vec<(Vector2<f32>, f32)>,
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
    pub fn new_py(initial_vehicle_state: VehicleState) -> Simulator {
        Simulator::new(initial_vehicle_state)
    }

    pub fn advance_s(&mut self, action: Action, delta_t_s: f32) -> f32 {
        self.advance(action, Duration::from_secs_f32(delta_t_s))
    }

    pub fn get_state_clone(&self) -> SimState {
        self.state.clone()
    }
}

impl Simulator {
    pub fn new(initial_vehicle_state: VehicleState) -> Simulator {
        Simulator {
            state: SimState {
                vehicle_state: initial_vehicle_state,
                reward_points: vec![
                    // (Vector2::new(10., 10.), 10.),
                    (Vector2::new(1000., 700.), 100.),
                ],
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
            |val: &(Vector2<f32>, f32)| (self.state.vehicle_state.position - val.0).norm() < 50.;

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
