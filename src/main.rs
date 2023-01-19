use std::thread;
use rand::Rng;
use crate::toll::Toll;
use crate::vehicle::Vehicle;

mod toll;
mod vehicle;
mod gate;

fn main() {
    let mut rng = rand::thread_rng();
    let mut toll = Toll::new(3);
    for gate in toll.gates.iter_mut() {
        gate.launch_thread();
    }
    loop {
        let vehicle = rng.gen::<Vehicle>();
        let time_until_next = vehicle.time_until_next(&mut rng);
        toll.add_vehicle(vehicle);
        println!("{}", &toll);
        println!("{:?}", &time_until_next);
        thread::sleep(time_until_next / 100);
    }
}
