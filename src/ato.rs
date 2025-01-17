use std::collections::HashMap;

use bveats_rs::{AtsHandles, AtsVehicleSpec, BveAts};

use crate::{pid::Pid, settings_load::get_settings_data};


#[derive(Debug, Default)]
pub struct ATO {
	vehicle_spec: AtsVehicleSpec,
	/// 信号Indexと制限速度の対応表
	signal_speed_table: HashMap<i32, f32>,
	/// 閉扉状態か
	door_closed: bool,
	/// ATO目標速度
	target_speed: f32,
	/// PID制御用
	pid: Pid,
}

impl BveAts for ATO {
	fn load(&mut self) {
		self.signal_speed_table = HashMap::from([
			(0, 0.0),	// 02信号
			(1, 0.0),	// 01信号
			(2, 15.0),	// 15信号
			(3, 25.0),	// 25信号
			(4, 45.0),	// 45信号
			(5, 60.0),	// 60信号
			(6, 75.0),	// 75信号
			(7, 90.0),	// 90信号
		]);
	}

	fn dispose(&mut self) { }

	fn set_vehicle_spec(&mut self, spec: bveats_rs::AtsVehicleSpec) {
		self.vehicle_spec = spec;
	}

	fn elapse(&mut self, state: bveats_rs::AtsVehicleState, _panel: &mut [i32], _sound: &mut [i32]) -> bveats_rs::AtsHandles {
		// 開扉状態なら常用最大
		if !self.door_closed {
			return AtsHandles {
				brake: self.vehicle_spec.brake_notches,
				power: 0,
				reverser: 0,
				constant_speed: 0
			}
		}
		let deceleration = self.pid.elapse(self.target_speed, state);
		let brake_notch = (deceleration / 3.5 * 31.0).clamp(-31.0, 0.0) as i32;
		let power_notch = (deceleration / 3.5 * 31.0).clamp(0.0, 31.0) as i32;

		AtsHandles {
			brake: brake_notch,
			power: power_notch,
			reverser: 1,
			constant_speed: 0
		}
	}

	fn key_down(&mut self, _key: bveats_rs::AtsKey) {
		
	}

	fn key_up(&mut self, _key: bveats_rs::AtsKey) {
		let settings = get_settings_data();
		self.pid.override_pid_constants(settings);
	}

	fn door_open(&mut self) {
		self.door_closed = false;
	}

	fn door_close(&mut self) {
		self.door_closed = true;
	}

	fn set_signal(&mut self, signal: i32) {
		let speed = self.signal_speed_table.get(&signal).copied().unwrap_or_default();
		self.target_speed = speed - 5.0;
	}

	fn set_beacon_data(&mut self, _data: bveats_rs::AtsBeaconData) {
		
	}

	// no-op
	fn initialize(&mut self, _handle: bveats_rs::AtsInit) { }
	fn set_power(&mut self, _notch: i32) { }
	fn set_brake(&mut self, _notch: i32) { }
	fn set_reverser(&mut self, _notch: i32) { }
	fn horn_blow(&mut self, _horn_type: bveats_rs::AtsHorn) { }
}