use lazy_static::lazy_static;
use rand::distributions::{Bernoulli, Standard};
use rand::prelude::*;
use rand_distr::{Geometric, Normal};
use std::cmp::min;
use std::time::Duration;
use crate::vehicle::paymen_mean::PaymentMean;
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::*;

#[derive(Debug)]
pub struct Vehicle {
    nb_passengers: u8,
    taxi: bool,
    low_carbon: bool,
    payment_mean: PaymentMean,
    type_: VehicleType,
    nb_kilometres: f32,
}

lazy_static!(
    // 1% des véhicules ont la vignette crit'air E
    static ref LOW_CARBON_RNG: Bernoulli = Bernoulli::new(0.01).unwrap();
    static ref NB_PASSENGER_RNG: Geometric = Geometric::new(0.5).unwrap();
    static ref TAXI_RNG: Bernoulli = Bernoulli::new(0.05).unwrap();
    static ref NB_KM_RNG_LIGHT: Normal<f32> = Normal::new(60.0, 10.0).unwrap();
    static ref NB_KM_RNG_HEAVY: Normal<f32> = Normal::new(76.0, 10.0).unwrap();
    static ref PAYMENT_TIME_RNG: Normal<f32> = Normal::new(60.0, 10.0).unwrap();
);

impl Distribution<Vehicle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vehicle {
        let vtype = rng.gen::<VehicleType>();
        let nb_passengers = match vtype {
            Light => min(8, 1 + NB_PASSENGER_RNG.sample(rng) as u8),
            _ => 1,
        };
        let low_carbon = match vtype {
            Light => LOW_CARBON_RNG.sample(rng),
            _ => false,
        };
        let taxi = match vtype {
            Light => TAXI_RNG.sample(rng),
            _ => false
        };
        let nb_kilometres = match vtype {
            Light | Motorcycle => NB_KM_RNG_LIGHT.sample(rng),
            _ => NB_KM_RNG_HEAVY.sample(rng),
        };
        Vehicle {
            nb_passengers,
            taxi,
            low_carbon,
            payment_mean: PaymentMean::rand_from_vehicle_type(rng, &vtype),
            type_: vtype,
            nb_kilometres,
        }
    }
}

impl Vehicle {
    #[inline(always)]
    pub fn carpooling(&self) -> bool {
        self.nb_passengers > 1 || self.taxi || self.low_carbon
    }

    pub fn payment_duration<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        let duration = Duration::from_secs(PAYMENT_TIME_RNG.sample(rng) as u64);
        match self.payment_mean {
            PaymentMean::Cash => duration,
            PaymentMean::Toll => duration / 2,
        }
    }

    pub fn type_num(&self) -> usize {
        self.type_ as usize
    }
}
