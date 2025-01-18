use nalgebra::Vector2;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::time::Duration;

#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[derive(Debug, Clone)]
pub struct VehicleState {
    pub position_x: f32,
    pub position_y: f32,
    pub speed: f32,
    pub angle: f32,
    pub base_length: f32,
    pub max_speed: f32,
}

impl Default for VehicleState {
    fn default() -> VehicleState {
        VehicleState {
            position_x: 0.,
            position_y: 0.,
            speed: 0.,
            angle: 0.,
            base_length: 100.,
            max_speed: 1000.,
        }
    }
}

// We will use a simple bicycle model for this, with the following dynamics equations:Vector2
// dx/dt     = v*cos(theta)
// dy/dt     = v*sin(theta)
// dtheta/dt = v*tan(deltoid)/L
// dv/dt     = a
//
// where:
// - x and y are the position of the vehicle in some reference frame
// - deltoid is the wheel steer angle
// - theta is the vehicle's actual angle
// - v is the linear velocity of the vehicle
// - a is the acceleration
// - L is the wheel base length, or distance between both wheels

#[cfg_attr(feature = "python", pymethods)]
impl VehicleState {
    #[cfg(feature = "python")]
    #[new]
    pub fn new() -> VehicleState {
        Self::default()
    }

    pub fn step(&mut self, acceleration: f32, wheel_steer_angle: f32, delta_t: Duration) {
        let delta_t_s = delta_t.as_secs_f32();
        // The order of modification is important to make sure things propagate correctly
        let position_delta =
            self.speed * Vector2::new(self.angle.cos(), self.angle.sin()) * delta_t_s;
        self.position_x += position_delta.x;
        self.position_y += position_delta.y;
        self.angle += self.speed * wheel_steer_angle.tan() / self.base_length * delta_t_s;
        self.speed += acceleration * delta_t_s;
        self.speed = self.speed.clamp(-self.max_speed, self.max_speed);
    }
}

impl VehicleState {
    pub fn position(&self) -> Vector2<f32> {
        Vector2::new(self.position_x, self.position_y)
    }
}
