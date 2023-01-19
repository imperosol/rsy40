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

    fn next_empty_gate(&mut self) -> Option<&Gate> {
        let gate = self.gates.iter_mut()
            .filter(|gate| gate.empty())
            .next();
        match gate {
            None => None,
            Some(gate) => Some(gate)
        }
    }

    #[inline(always)]
    fn has_empty_gate(&self) -> bool {
        self.gates.iter().any(|g| g.empty())
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        let less_crowded_gate = self.gates.iter()
            .filter(|&gate| !gate.empty())
            .min_by(|a, b| a.nb_cars().cmp(&b.nb_cars()));
        let gate = match less_crowded_gate {
            None => self.next_empty_gate().unwrap(),
            Some(gate) => {
                if gate.nb_cars() >= 5 && self.has_empty_gate() {
                    self.next_empty_gate().unwrap()
                } else {
                    gate
                }
            }
        };
        gate.queue
            .lock()
            .unwrap()
            .push_back(vehicle);
        gate.cond.notify_all();
    }
}

impl Display for Toll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for (i, gate) in self.gates.iter().enumerate() {
            buffer.push_str(i.to_string().as_str());
            buffer.push_str(" | ");
            if gate.empty() {
                buffer.push('X');
            }
            for v in gate.queue.lock().unwrap().iter() {
                buffer.push_str(v.type_num().to_string().as_str());
            }
            buffer.push('\n');
        }
        f.write_str(buffer.as_str())
    }
}