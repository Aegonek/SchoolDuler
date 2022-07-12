#![allow(dead_code)]

use std::error::Error;

use algen::parametrized::algorithm::Algorithm;
use algen::solutions::theta;
use domain::{Course, Schedule};
use once_cell::sync::OnceCell;
use utils::log::log_item;

use crate::algen::parametrized::execution::RunId;
use crate::algen::solutions::theta::{config::Config, TerminationCondition};
use crate::db::DB_CONN;

mod algen;
mod domain;
mod db;
mod utils;

pub static RUN_ID: OnceCell<RunId> = OnceCell::new();

fn main() -> Result<(), Box<dyn Error>> {
    let run_id = {
        let nmb: usize = DB_CONN.lock().unwrap().query_row("
            SELECT run FROM THETA_ITERATIONS 
            ORDER BY run DESC
            LIMIT 1
        ", [], |row| row.get(0)).unwrap_or(0);
        RunId(nmb + 1)
    };
    RUN_ID.set(run_id).unwrap();

    let raw = include_str!("../input/example-courses.json");

    println!("Reading input requirements...");
    let courses: Vec<Course> = serde_json::from_str(raw)?;
    println!("Generating solution...");
    let solver = theta::Solution::with_config(Config { termination_condition: TerminationCondition::AfterNoIterations(1000), ..Config::default() });
    let schedule = solver.run(&courses);

    println!("Generated solution!");
    // TODO: instance for Vec<Schedule>
    for class in schedule.iter() {
        log_item(class);
    }
    save_to_xlsx(&schedule);

    Ok(())
}

fn save_to_xlsx(_schedule: &Schedule) {
    println!("Writing to xlsx not yet implemented.")
}