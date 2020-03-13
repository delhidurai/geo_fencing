// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///
/// Structs and Functions that are commonly used across multiple module will reside in this
/// library
///
extern crate ansi_term;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde_derive::Deserialize;
//use std::time::Instant;
//use std::time::Duration;

///Types of Shapes handled in this programm
#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Polygon,
    Circle,
    Triangle,
}

pub enum Colour {
    Red,
    Blue,
    Yellow,
    Purple,
    Black,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

impl Coordinates {
    pub fn new(lat: f64, lon: f64) -> Coordinates {
        Coordinates { lat, lon }
    }
    pub fn display(self) {
        println!(
            "Coordinates Latitude {} and Longitude {} ",
            self.lat, self.lon
        );
    }
}

#[derive(Deserialize, Debug)]
pub struct CircleCoordinates {
    pub lat: f64,
    pub lon: f64,
    pub rad: f64,
}

impl CircleCoordinates {
    pub fn new(lat: f64, lon: f64, rad: f64) -> CircleCoordinates {
        CircleCoordinates { lat, lon, rad }
    }
    pub fn display(self) {
        println!(
            "Circle Coordinates Latitude {} , Longitude {} and Radius {} ",
            self.lat, self.lon, self.rad
        );
    }
}

#[derive(Deserialize, Debug)]
pub struct MovingTracker {
    pub _comment: String,
    pub vehicle: String,
    pub shape: String,
    pub shape_coordinate: Vec<Coordinates>,
    pub moving_coordinate: Vec<Coordinates>,
}

#[derive(Deserialize, Debug)]
pub struct MovingTrackerCircle {
    pub _comment: String,
    pub vehicle: String,
    pub shape: String,
    pub shape_coordinate: CircleCoordinates,
    pub moving_coordinate: Vec<Coordinates>,
}

pub fn display_underline(str: &str, colour: Colour) {
    match colour {
        Colour::Red => println!("{}", ansi_term::Colour::Red.bold().underline().paint(str)),
        Colour::Yellow => println!(
            "{}",
            ansi_term::Colour::Yellow.bold().underline().paint(str)
        ),
        Colour::Blue => println!("{}", ansi_term::Colour::Blue.bold().underline().paint(str)),
        Colour::Purple => println!(
            "{}",
            ansi_term::Colour::Purple.bold().underline().paint(str)
        ),
        Colour::Black => println!("{}", ansi_term::Colour::Black.bold().underline().paint(str)),
    }
}

pub fn display_bold(str: &str, colour: Colour) {
    match colour {
        Colour::Red => println!("{}", ansi_term::Colour::Red.bold().paint(str)),
        Colour::Yellow => println!("{}", ansi_term::Colour::Yellow.bold().paint(str)),
        Colour::Blue => println!("{}", ansi_term::Colour::Blue.bold().paint(str)),
        Colour::Purple => println!("{}", ansi_term::Colour::Purple.bold().paint(str)),
        Colour::Black => println!("{}", ansi_term::Colour::Black.bold().paint(str)),
    }
}

pub fn display(str: &str, colour: Colour) {
    match colour {
        Colour::Red => println!("{}", ansi_term::Colour::Red.paint(str)),
        Colour::Yellow => println!("{}", ansi_term::Colour::Yellow.paint(str)),
        Colour::Blue => println!("{}", ansi_term::Colour::Blue.paint(str)),
        Colour::Purple => println!("{}", ansi_term::Colour::Purple.paint(str)),
        Colour::Black => println!("{}", str),
    }
}

//pub fn get_current_time()->Instant{
//    Instant::now()
//}
//pub fn calculate_run_time(instant:&Instant)->Duration{
//    instant.elapsed()
//}
