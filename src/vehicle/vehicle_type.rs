use crate::vehicle::vehicle_type::VehicleType::*;
use lazy_static::lazy_static;
use rand::distributions::{Standard, WeightedIndex};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum VehicleType {
    Light = 0,
    Medium,
    Truck,
    HeavyTruck,
    Motorcycle,
}

lazy_static!(
    static ref DIST: WeightedIndex<i32> = WeightedIndex::new(&[80, 10, 5, 4, 1]).unwrap();
);

impl Distribution<VehicleType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VehicleType {
        static TYPES: [VehicleType; 5] = [Light, Medium, Truck, HeavyTruck, Motorcycle];
        TYPES[DIST.sample(rng)]
    }
}
