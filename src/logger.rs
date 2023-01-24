use std::fs::{remove_file};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use crate::gate::{DepartedVehicle};

/// Gère l'enregistrement des voitures en base de données
/// grâce à un modèle MPSC
pub struct TollDatabase {
    /// objet Sender utilisé pour envoyer des données
    /// au thread d'enregistrement en db.
    pub sender: Sender<DepartedVehicle>,
}

impl TollDatabase {
    /// Lance le thread d'enregistrement en db puis
    /// renvoie un nouvel objet TollDatabase
    pub fn new(db_name: &str) -> Result<Self, ()> {
        if db_name != ":memory:" {
            let path = Path::new(db_name);
            if path.exists() {
                match path.is_file() {
                    true => remove_file(path).unwrap(),
                    false => return Err(())
                };
            }
        }
        match sqlite::open(db_name) {
            Err(_) => Err(()),
            Ok(conn) => {
                create_table(&conn);
                Ok(Self { sender: launch_log_thread(conn) })
            }
        }
    }
}

fn launch_log_thread(conn: sqlite::Connection) -> Sender<DepartedVehicle> {
    let (rx, tx) = channel();
    thread::spawn(move || {
        loop {
            log_vehicle(&conn, tx.recv().unwrap());
        }
    });
    rx
}

fn create_table(conn: &sqlite::Connection) {
    let query = "\
        create table vehicle ( \
            id            INTEGER not null \
                constraint id \
                    primary key autoincrement, \
            kilometres    INTEGER not null, \
            nb_passengers INTEGER not null, \
            type          INTEGER not null, \
            payment_mean  INTEGER not null, \
            arrival       TEXT    not null, \
            departure     TEXT    not null, \
            constraint type_check_1 \
                check (type >= 0), \
            constraint type_check_2 \
                check (type < 6) \
        ); \
        create unique index vehicle_id_uindex \
            on vehicle (id);";
    conn.execute(query).unwrap();
}

fn log_vehicle(conn: &sqlite::Connection, v: DepartedVehicle) {
    let query = format!(
        "insert into vehicle (\
            kilometres, nb_passengers, type, \
            payment_mean, arrival, departure\
        ) values ({}, {}, {}, {}, \"{}\", \"{}\");",
        v.vehicle.nb_kilometres,
        v.vehicle.nb_passengers,
        v.vehicle.type_num(),
        v.vehicle.payment_mean as u32,
        v.arrival.to_timestamp(),
        v.departure.to_timestamp()
    );
    conn.execute(query.as_str()).unwrap();
}
