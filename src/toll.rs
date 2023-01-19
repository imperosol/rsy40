use std::fmt::{Debug, Display, Formatter};
use crate::gate::Gate;
use crate::vehicle::Vehicle;

#[derive(Debug)]
pub struct Toll {
    pub gates: Vec<Gate>,
}

impl Toll {
    pub fn new(nb_gates: usize) -> Self {
        let mut gates = Vec::new();
        for _ in 0..nb_gates {
            gates.push(Gate::new())
        }
        Self { gates }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        let less_crowded_gate: &Gate = self.gates.iter()
            .min_by(|a, b| a.nb_cars().cmp(&b.nb_cars()))
            .unwrap();
        less_crowded_gate.queue
            .lock()
            .unwrap()
            .push_back(vehicle);
        less_crowded_gate.cond.notify_all();
    }
}

impl Display for Toll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for (i, gate) in self.gates.iter().enumerate() {
            buffer.push_str(i.to_string().as_str());
            buffer.push_str(" | ");
            for v in gate.queue.lock().unwrap().iter() {
                buffer.push_str(v.type_num().to_string().as_str());
            }
            buffer.push('\n');
        }
        f.write_str(buffer.as_str())
    }
}