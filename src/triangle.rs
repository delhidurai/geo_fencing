// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///
/// This module will create Triangular fence and will validate if the given coordinates is within the fence or not
///
use coordinates::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::{thread, time};

// Triangle
fn get_area(side1: &Coordinates, side2: &Coordinates, side3: &Coordinates) -> f64 {
    let area = (side1.lat * (side2.lon - side3.lon))
        + (side2.lat * (side3.lon - side1.lon))
        + (side3.lat * (side1.lon - side2.lon));
    area.abs()
}

///
/// This method checks if a particular coordinate is within the triangular fence.
/// This method  verifies if a line drawn from a point is intersecting
/// within the boundaries of the created fence.
/// It returns boolean true if the coordinates are within the fence and
/// false if the coordinates are outside the fence.
///

pub fn contains(triangle: &[Coordinates], point: &Coordinates) -> bool {
    let mut b_contains = false;
    if triangle.len() == 3 {
        let side1 = &triangle[0];
        let side2 = &triangle[1];
        let side3 = &triangle[2];
        //Calculate area of triangle ABC
        let area_abc = get_area(side1, side2, side3);
        println!("area {:?}", area_abc);
        //Calculate area of triangle PBC
        let area_pbc = get_area(point, side2, side3);
        //Calculate area of triangle PAC
        let area_pac = get_area(side1, point, side3);
        //Calculate area of triangle PAB
        let area_pab = get_area(side1, side2, point);
        // Check if sum of area_pbc, area_pac and area_pab  is equal to area_abc
        if (area_abc - (area_pab + area_pac + area_pbc)).abs() == 0.0 {
            b_contains = true;
        }
    }
    b_contains
}

/// The below function reads the incoming json file which is of the format
///{
//  "shape": "Triangle",
//  "vehicle": "van",
//  "shape_coordinate":[ {"lat": -2.0,"lon": 3.0},{"lat": 4.0,"lon": 4.0},
//  "moving_coordinate": [{"lat":1.0 ,"lon": 1.0}]
//}
/// shape_coordinate -> contains the coordinates which is used to create the Triangular fence
/// moving_coordinate -> contains series of lat, lon for which location has to be determined.
/// maps it to the MovingTracker struct and returns the result to the caller.
///The function the below parameters
/// filename -> contains the json file which contains the coordinates
/// Returns Result -> contains either MovingTracker struct if success or Error if any failure.
pub fn read_movingtracker_file(filename: &str) -> Result<MovingTracker, Box<dyn Error>> {
    // Open the file in read-only mode.

    let current_path = env::current_dir();
    let mut path_buff = current_path.ok().unwrap();
    path_buff.push("data");
    path_buff.push(filename);
    let actual_path = path_buff.to_str().unwrap();

    // Read the JSON contents of the file as an instance of `MovingTracker`.
    let u = serde_json::from_reader(File::open(actual_path)?)?;

    // Return the value as Result.
    Ok(u)
}

/// The below function calls get_json_info  retrieves MovingTracker struct
/// The function takes the below parameters
/// filename -> contains the json file which contains the coordinates
/// Returns MovingTracker
pub fn get_json_info(filename: &str) -> MovingTracker {
    // Read the JSON contents of the file as an instance of `MovingTracker`.

    let json_structure = read_movingtracker_file(filename);
    match json_structure{
        Ok(json_structure)=> json_structure,
        Err(error) => {
            panic!("Problem opening the json file. Check if the file is correct and has correct json values. The error is : {:?}", error)
        }
    }
}
/// The function does the below
/// Reads the input file
/// Creates triangular fence based on the coordinates given in the input file
/// Executes set of coordinates for which position has to be determined.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates
///
pub fn execute_triangle(filename: &str, delay: bool) -> String {
    let second = time::Duration::from_millis(1000);
    //let start = get_current_time();
    let u = get_json_info(filename);
    let cor: Vec<Coordinates> = u.shape_coordinate;
    let run: Vec<Coordinates> = u.moving_coordinate;
    let mut ret_val = "".to_string();
    display_bold(
        "Created Triangular Fence, with the below coordinates",
        Colour::Blue,
    );
    display(&format!("{:?}", cor), Colour::Blue);
    display_bold("Tracking and Verifying, if the below moving objects position is within or outside the fence", Colour::Black);

    for x in run {
        let mut str = "is out of the fence";
        if delay {
            thread::sleep(second);
        }

        if contains(&cor, &x) {
            str = "is inside the fence";
        }
        display(
            &format!(
                "The {} positioned at latitude {}, longitude {}, {} ",
                u.vehicle, x.lat, x.lon, &str
            ),
            Colour::Black,
        );
        ret_val.push_str(
            format!(
                "The {} positioned at latitude {}, longitude {}, {} \n ",
                u.vehicle, x.lat, x.lon, &str
            )
            .as_str(),
        );
    }
    //  println!("The time taken for the execute method to perform this operation is {:?}", calculate_run_time(&start));
    ret_val
}

/// The function does the below
/// Reads the input file
/// Creates triangular fence based on the coordinates given in the input file
/// checks if the given coordinates in the function paramater is within the circular fence.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates which is necessary to create the fence
/// lat and lon -> search coordinates
pub fn contains_in_triangle(filename: &str, lat: f64, lon: f64) -> bool {
    // let start = get_current_time();
    let u = get_json_info(filename);
    let cor: Vec<Coordinates> = u.shape_coordinate;
    let point: Coordinates = Coordinates::new(lat, lon);
    let mut retval = false;
    display_bold(
        "Searching the vehicle in Polygon Fence, which is built with coordinates",
        Colour::Blue,
    );
    display(&format!("{:?}", cor), Colour::Blue);
    let mut str = "is out of the fence";
    if contains(&cor, &point) {
        str = "is inside the fence";
        retval = true;
    }
    display(
        &format!(
            "The {} positioned at latitude {}, longitude {}, {} ",
            u.vehicle, lat, lon, &str
        ),
        Colour::Black,
    );
    //  println!("The time taken for the execute method to perform this operation is {:?}", calculate_run_time(&start));
    retval
}
