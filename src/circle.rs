// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use coordinates::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::{thread, time};

/// The below function checks if the given coordinates latitude and longitude are inside the fence created with CircleCoordinates
/// for the given radius.
/// It uses Pythagoras theorem to calculate if a given point is within the circular fence created
/// The function the below parameters
/// CircleCoordinates -> contains Coordinates required for creating circular fence
/// Coordinates -> contains coordinates that for which location has to be determined.
/// Returns bool -> True if the coordinates are inside the fence, False if it is outside.
///
fn contains(circle: &CircleCoordinates, point: &Coordinates) -> bool {
    let mut b_return = false;
    // use Pythagoras theorem to figure out if the given latitude and longitude is within circle for the given radius
    // (x-center_x)^2 + (y - center_y)^2 < radius^2
    let distance = f64::powf(circle.lat - point.lat, 2.0) + f64::powf(circle.lon - point.lon, 2.0);
    if distance <= (f64::powf(circle.rad, 2.0)) {
        b_return = true;
    }
    b_return
}

/// The below function reads the incoming json file which is of the format
///{
//  "shape": "Circle",
//  "vehicle": "van",
//  "shape_coordinate":{"lat":1.0 ,"lon": 1.0,"rad": 6.0 },
//  "moving_coordinate": [{"lat":1.0 ,"lon": 1.0}]
//}
/// shape_coordinate -> contains the coordinates which is used to create the circular fence
/// moving_coordinate -> contains series of lat, lon for which location has to be determined.
/// maps it to the MovingTrackerCircle struct and returns the result to the caller.
///The function the below parameters
/// filename -> contains the json file which contains the coordinates
/// Returns Result -> contains either MovingTrackerCircle struct if success or Error if any failure.
fn read_movingtrackercircle_file(filename: &str) -> Result<MovingTrackerCircle, Box<dyn Error>> {
    // Open the file in read-only mode.
    let current_path = env::current_dir();
    let mut path_buff = current_path.ok().unwrap();
    path_buff.push("data");
    path_buff.push(filename);
    let actual_path = path_buff.to_str().unwrap();

    // Read the JSON contents of the file as an instance of `MovingTrackerPolygon`.
    let u = serde_json::from_reader(File::open(actual_path)?)?;

    // Return the value as Result.
    Ok(u)
}
/// The below function calls get_circle_json_info  retrieves MovingTrackerCircle struct
///The function takes the below parameters
/// filename -> contains the json file which contains the coordinates
/// Returns MovingTrackerCircle
fn get_circle_json_info(filename: &str) -> MovingTrackerCircle {
    // Read the JSON contents of the file as an instance of `MovingTrackerPolygon`.

    let json_structure = read_movingtrackercircle_file(filename);
    match json_structure{
        Ok(json_structure)=> json_structure,
        Err(error) => {
            panic!("Problem opening the json file. Check if the file is correct and has correct json values. The error is : {:?}", error)
        }
    }
}

/// The function does the below
/// Reads the input file
/// Creates circle fence based on the coordinates given in the input file
/// Executes set of coordinates for which position has to be determined.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates

pub fn execute_circle(filename: &str, delay: bool) -> String {
    let second = time::Duration::from_millis(1000);
    //   let start = get_current_time();
    let u = get_circle_json_info(filename);
    let cor: CircleCoordinates = u.shape_coordinate;
    let run: Vec<Coordinates> = u.moving_coordinate;
    let mut ret_val = "".to_string();
    display_bold(
        "Created Circular Fence, with the  coordinates",
        Colour::Blue,
    );
    display(
        &format!(
            "latitude {}, longitude {}, radius {}",
            cor.lat, cor.lon, cor.rad
        ),
        Colour::Blue,
    );

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
/// Creates circle fence based on the coordinates given in the input file
/// checks if the given coordinates in the function paramater is within the circular fence.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates which is necessary to create the fence
/// lat and lon -> search coordinates

pub fn contains_in_circle(filename: &str, lat: f64, lon: f64) -> bool {
    //  let start = get_current_time();
    let u = get_circle_json_info(filename);
    let cor: CircleCoordinates = u.shape_coordinate;
    let point: Coordinates = Coordinates::new(lat, lon);
    let mut retval = false;
    display_bold(
        "Searching the vehicle in Circular Fence, which is built with coordinates",
        Colour::Blue,
    );
    display(
        &format!(
            "latitude {}, longitude {}, radius {}",
            cor.lat, cor.lon, cor.rad
        ),
        Colour::Blue,
    );
    //println!("******** Searching the vehicle in Circular Fence, which is built with coordinates, latitude {}, longitude {}, radius {} *********", cor.lat,cor.lon,cor.rad);
    let mut str = "is out of the fence";
    if contains(&cor, &point) {
        str = "is inside the fence";
        retval = true
    }
    display(
        &format!(
            "The {} positioned at latitude {}, longitude {}, {} ",
            u.vehicle, lat, lon, &str
        ),
        Colour::Black,
    );
    // println!("The time taken for the execute method to perform this operation is {:?}", calculate_run_time(&start));
    //println!("The {} positioned at latitude {}, longitude {}, {} ",u.vehicle, lat,lon, &str);
    retval
}
