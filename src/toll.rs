use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;
use rand::Rng;
use rand::prelude::*;
use crate::gate::{Gate, WaitingVehicle};
use crate::toll_clock::TollClock;
use crate::vehicle::Vehicle;

#[derive(Debug)]
pub struct Toll {
    pub gates: Vec<Gate>,
    pub clock: TollClock
}

impl Toll {
    pub fn builder() -> TollBuilder {
        TollBuilder::default()
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        let gates = match vehicle.carpooling() {
            true => self.gates.as_slice(),
            false => self.gates.split_last().unwrap().1
        };
        let less_crowded_gate = gates.iter()
            .filter(|&gate| !gate.empty())
            .min_by(|a, b| a.nb_cars().cmp(&b.nb_cars()));
        let gate = match less_crowded_gate {
            None => gates.get(0).unwrap(),
            Some(gate) => gates.iter()
                .find(|gate| gate.empty())
                .unwrap_or(gate),
        };
        gate.queue
            .lock()
            .unwrap()
            .push_back(WaitingVehicle {
                vehicle,
                arrival: self.clock.clone(),
            });
        gate.cond.notify_all();
    }

    pub fn time_until_next_vehicle<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        static LAMBDAS: [f64; 24] = [
            0.0125, 0.01, 0.00909, 0.0125, 0.0125, 0.014285,
            0.033333, 0.066666, 0.232558, 0.2, 0.05, 0.04,
            0.04, 0.033333, 0.028571, 0.04, 0.1, 0.25,
            0.1, 0.033333, 0.025, 0.02, 0.016666, 0.014285
        ];
        let current_lambda = LAMBDAS[self.clock.clock.hour as usize];
        let generator = rand_distr::Exp::new(current_lambda).unwrap();
        Duration::from_secs((generator.sample(rng)) as u64)
    }
}

impl Display for Toll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = self.clock.clock.to_string();
        for (i, gate) in self.gates.iter().enumerate() {
            buffer.push_str(i.to_string().as_str());
            buffer.push_str(" | ");
            if gate.empty() {
                buffer.push('X');
            }
            for v in gate.queue.lock().unwrap().iter() {
                buffer.push_str(v.vehicle.type_num().to_string().as_str());
            }
            buffer.push('\n');
        }
        f.write_str(buffer.as_str())
    }
}

#[derive(Default)]
pub struct TollBuilder {
    gates: Vec<Gate>,
    clock: TollClock
}

impl TollBuilder {
    pub fn build(self) -> Toll {
        Toll {
            gates: self.gates,
            clock: self.clock,
        }
    }

    pub fn nb_gates(mut self, nb_gates: usize) -> Self {
        if !self.gates.is_empty() {
            self.gates.clear();
        }
        self.gates.extend((0..nb_gates).map(|_| Gate::new()));
        self
    }

    pub fn start_hour(mut self, hour: TollClock) -> Self {
        self.clock = hour;
        self
    }

    pub fn acceleration_factor(mut self, factor: u32) -> Self {
        self.clock.acceleration_factor = factor;
        self
    }
}