use lazy_static::lazy_static;
use rand::distributions::Bernoulli;
use rand::prelude::*;
use crate::vehicle::paymen_mean::PaymentMean::{Cash, Toll};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::*;

#[derive(Debug)]
pub enum PaymentMean {
    Cash,
    Toll, // télépéage
}

lazy_static!(
    static ref RNG_LIGHT: Bernoulli = Bernoulli::new(0.52).unwrap();
    static ref RNG_HEAVY: Bernoulli = Bernoulli::new(0.94).unwrap();
);

impl PaymentMean {
    /// Le type de paiement est généré aléatoirement suivant une loi de Bernoulli
    /// - les véhicules légers et les motos ont une probabilité de 0.52 d'avoir le télépéage
    /// - les autres véhicules ont une probablité de 0.94 d'avoir le télépéage
    pub fn rand_from_vehicle_type<R: Rng + ?Sized>(rng: &mut R, vtype: &VehicleType) -> Self {
        match vtype {
            Light | Motorcycle => match RNG_LIGHT.sample(rng) {
                true => Toll,
                false => Cash,
            },
            _ => match RNG_HEAVY.sample(rng) {
                true => Toll,
                false => Cash,
            },
        }
    }
}
