// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///
/// The geofence_simulator provides a real time simulation of two scenarios in geofencing
/// Simulation 1:- Validate if a moving object is in or out of the geo fence
/// Simulation 2:- Validate if a user provided coordinate is in  or out of geo fence
/// The data required to create Geo fence and the moving objects coordinates should be listed in the
/// data/ *.json files
/// Example
/// for Simulation 1, the json file should contain shape_coordinate and moving_coordinate information.
/// For Simulation 2, the json file should contain just shape_coordinate and an empty moving_coordinate
///
///
extern crate geofencing;

use geofencing::coordinates::*;
use geofencing::geofencer::*;
use std::io::stdin;

///
/// Retrieves command line input from the user
///
pub fn get_user_input(str: &mut String) {
    match stdin().read_line(str) {
        Ok(_) => {
            //do nothing
        }
        Err(e) => {
            panic!("Unable to read the user input {} ", e);
        }
    }
}

///
/// Methods to print user instruction
///
fn user_display(_str: &str) {
    display_bold(&format!("YOU HAVE SELECTED {}", _str), Colour::Black);
    display_bold(
        "PLACE THE FILE IN geofencing/data FOLDER AND PROVIDE THE FILE NAME",
        Colour::Black,
    );
    display_bold("REFER TO README FILE FOR FILE STRUCTURE", Colour::Black);
}

///
/// Methods to print user instruction
///
fn user_title(_str: &str, _desc: &str) {
    display_underline(&format!("SIMULATION {}:-", _str), Colour::Purple);
    display_underline(
        &format!("GEO LOCATION SIMULATION FOR {}.", _desc),
        Colour::Purple,
    );
    display_bold(
        "SELECT THE SHAPE THAT YOU WANT YOUR FENCE TO BE",
        Colour::Blue,
    );
    display_bold("1. POLYGON  ", Colour::Blue);
    display_bold("2. CIRCLE  ", Colour::Blue);
    display_bold("3. TRIANGLE   ", Colour::Blue);
}

///
/// This function is the starting point
/// This function will simulate two scenarios
/// Simulation 1:- Validate if a moving object is in or out of the geo fence
/// Simulation 2:- Validate if a user provided coordinate is in  or out of geo fence
/// The data required to create Geo fence and the moving objects coordinates should be listed in the
/// data/ *.json files
/// Example
/// for Simulation 1, the json file should contain shape_coordinate and moving_coordinate information.
/// For Simulation 2, the json file should contain just shape_coordinate and an empty moving_coordinate
///
///
pub fn main() {
    // Simulation 1 :- Verify if a moving target (Intransit object)  is with in the Fence

    user_title("1", "IN-TRANSIT OBJECTS");
    let mut input = String::new();
    get_user_input(&mut input);
    let mut int_value: i32 = input.trim().parse().unwrap_or(0);
    // let start = get_current_time();
    input = String::new();
    if int_value == 1 {
        user_display("POLYGON");
        get_user_input(&mut input);
        input.pop();
        execute(Shape::Polygon, input.as_str(), true);


    } else if int_value == 2 {
        user_display("CIRCLE");
        get_user_input(&mut input);
        input.pop();
        execute(Shape::Circle, input.as_str(), true);
    } else if int_value == 3 {
        user_display("TRIANGLE");
        get_user_input(&mut input);
        input.pop();
        execute(Shape::Triangle, input.as_str(), true);
    } else {
        display_bold(
            "THE SELECTION IS NOT IN THE LIST. PLEASE TRY AGAIN",
            Colour::Red,
        );
    }
    //    let duration = calculate_run_time(&start);
    //    println!("Time elapsed  is: {:?}", duration);
    // Simulation 2 :- Verify if a particular coordinate is with in the Fence

    user_title("2", "PARTICULAR COORDINATE");
    input = String::new();
    get_user_input(&mut input);
    int_value = input.trim().parse().unwrap_or(0);
    let mut filename = String::new();
    let mut shape: Shape = Shape::Polygon;
    let mut str_shape: &str = "POLYGON";
    if int_value == 1 {
        user_display(str_shape);
        get_user_input(&mut filename);
        filename.pop();
    } else if int_value == 2 {
        str_shape = "CIRCLE";
        shape = Shape::Circle;
        user_display(str_shape);
        get_user_input(&mut filename);
        filename.pop();
    } else if int_value == 3 {
        str_shape = "TRIANGLE";
        shape = Shape::Triangle;
        user_display(str_shape);
        get_user_input(&mut filename);
        filename.pop();
    } else {
        display_bold(
            "THE SELECTION IS NOT IN THE LIST. PLEASE TRY AGAIN",
            Colour::Red,
        );
    }
    if int_value != 0 {
        display_underline(str_shape, Colour::Blue);
        let mut done = false;
        while !done {
            display(
                "Enter the Latitude (defaulted to 0 if incorrect value) ",
                Colour::Purple,
            );
            input = String::new();
            get_user_input(&mut input);
            let int_lat: f64 = input.trim().parse().unwrap_or(0.0) as f64;
            display(
                "Enter the Longitude (defaulted to 0 if incorrect value)",
                Colour::Purple,
            );
            input = String::new();
            get_user_input(&mut input);
            let int_lon: f64 = input.trim().parse().unwrap_or(0.0) as f64;
            display(
                "CHECKING IF THE GIVEN COORDINATE IS WITHIN THE FENCE ",
                Colour::Purple,
            );
            contains(shape, filename.as_str(), int_lat, int_lon);
            display(
                "Do you want to continue?. Y to continue or any other key to exit ",
                Colour::Purple,
            );
            input = String::new();
            get_user_input(&mut input);
            input.pop();
            if input.as_str() != "Y" {
                done = true;
            }
        }
    }
}
