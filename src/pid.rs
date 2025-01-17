use bveats_rs::AtsVehicleState;
use serde::{Deserialize, Serialize};

use crate::timer::Timer;

#[derive(Debug, Default)]
pub struct Pid
{
	timer: Timer,

	/// 積分項
	integral: f32,
	/// 
	previous_state: AtsVehicleState,
	previous_error: f32,
	previous_deceleration: f32,

	/// 比例ゲイン
	kp: f32,
	/// 積分ゲイン
	ki: f32,
	/// 微分ゲイン
	kd: f32,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct PidConstants {
	kp: f32,
	ki: f32,
	kd: f32,
}

impl Pid {
	pub fn new() -> Self {
		Self {
			kp: 1.0,
			ki: 0.0,
			kd: 0.0,
			timer: Timer::new(200),
			..Default::default()
		}
	}

	pub fn elapse(&mut self, target_speed: f32, state: AtsVehicleState) -> f32 {
		if self.timer.is_ready(state.time) {
			let delta = (state.time - self.previous_state.time) as f32 * 1000.0;
			let error = target_speed - state.speed;
			self.integral += error * delta;
			let derivative = (error - self.previous_error) * delta;

			let result_deceleration = self.kp * error + self.ki * self.integral + self.kd * derivative;

			self.previous_error = error;
			self.previous_state = state;
			self.previous_deceleration = result_deceleration;
		}
		self.previous_deceleration
	}

	pub fn override_pid_constants(&mut self, constants: PidConstants) {
		self.kp = constants.kp;
		self.ki = constants.ki;
		self.kd = constants.kd;
	}
}