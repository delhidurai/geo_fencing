// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// This module will create polygon fence and will validate if the given coordinates is within the fence or not
/// Uses Ray casting along with Cramers rule to verify if the line intersects or not.
///
use coordinates::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::{thread, time};

/// The below function will form list of line equations between various points (Coordinates).
/// The last point will be circled back to the first point.
/// For example if P, Q, R are three different points then
/// Line equation will be PQ, QR, RP
///
///

fn get_lineequation(points: &[Coordinates], len: usize) -> Vec<(f64, f64, f64)> {
    let mut line_equn: Vec<(f64, f64, f64)> = Vec::new();

    for (_pos, coord) in points.iter().enumerate() {
        let point1 = coord;
        let point2: &Coordinates = &(points[((_pos + 1) % len)]);
        //Check if vertical or horizontal line first.
        if (point1.lon - point2.lon).abs() == 0.0 {
            //Vertical
            line_equn.push((0.0, 1.0, point1.lon));
        } else if (point1.lat - point2.lat).abs() == 0.0 {
            //Horizontal
            line_equn.push((1.0, 0.0, point1.lat));
        } else {
            //Not vertical or Horizontal line
            let a = -1.0 * ((point2.lon - point1.lon) / (point2.lat - point1.lat));
            let c = point1.lon + (a * point1.lat);
            line_equn.push((a, 1.0, c));
        }
    }
    //println!("The line equation {:?}",line_equn);
    line_equn
}

/// This method will check if the a given lat or lon is within a particular
/// coordinate.
///
fn check_if_within(point: f64, point1: f64, point2: f64) -> bool {
    point1 >= point2 && point <= point1 && point >= point2 || point <= point2 && point >= point1
}

/// This method will check if the a given lat or lon is within a particular
/// coordinate.
///
fn check_inbounds(point: &Coordinates, point1: &Coordinates, point2: &Coordinates) -> bool {
    let within_x = check_if_within(point.lat, point1.lat, point2.lat);
    let within_y = check_if_within(point.lon, point1.lon, point2.lon);
    let mut retval = false;
    if within_x && within_y {
        retval = true
    }

    retval
}
///
/// This method checks if a particular value is within the list
///
fn vec_contains(point: &Coordinates, vec_coord: &[Coordinates]) -> bool {
    let mut retval = false;
    for (_pos, coord) in vec_coord.iter().enumerate() {
        if (point.lat - coord.lat).abs() == 0.0 && (point.lon - coord.lon).abs() == 0.0 {
            retval = true;
            break;
        }
    }
    retval
}

///
/// This method checks if a particular coordinate is within the polygon fence.
/// This method uses Cramer's rule in verifying if a line drawn from a point is intersecting
/// within the boundaries of the created fence.
/// It returns boolean true if the coordinates are within the fence and
/// false if the coordinates are outside the fence.
///

fn contains(point: &Coordinates, points: &[Coordinates]) -> bool {
    // Find the horizontal line equation that passes through the point.
    // Use the Ax + By = C format and depict as a tuple of (A, B, C).

    let point_horizon_eqn: (f64, f64, f64) = (0.0, 1.0, point.lon);
    let points_len = points.len();
    let mut retval = false;
    if points.len() < 3 {
        panic!("The supplied fence points should be more than 3 for creating proper polygon");
    }
    let line_eq: Vec<(f64, f64, f64)> = get_lineequation(&points, points_len);
    let mut intersection_left: Vec<Coordinates> = Vec::new();
    let mut intersection_right: Vec<Coordinates> = Vec::new();
    for (_pos, coord) in line_eq.iter().enumerate() {
        // find_intersect() function uses Cramers rule to verify if the line intersects or not.
        let det1 = (coord.0 * point_horizon_eqn.1) - (coord.1 * point_horizon_eqn.0);
        let detx = (coord.2 * point_horizon_eqn.1) - (coord.1 * point_horizon_eqn.2);
        let dety = (coord.0 * point_horizon_eqn.2) - (coord.2 * point_horizon_eqn.0);

        if det1 != 0.0 {
            let point_coordinate = Coordinates::new(detx / det1, dety / det1);
            let point1 = &points[_pos];
            let point2 = &(points[((_pos + 1) % points_len)]);
            let check_inbound: bool = check_inbounds(&point_coordinate, point1, point2);
            if point_coordinate.lat < point.lat {
                if !vec_contains(&point_coordinate, &intersection_left) && check_inbound {
                    intersection_left.push(point_coordinate);
                }
            } else if !vec_contains(&point_coordinate, &intersection_right) && check_inbound {
                intersection_right.push(point_coordinate);
            }
        }
    }

    if (!intersection_left.is_empty() && !intersection_right.is_empty())
        && (intersection_left.len() % 2 == 1 && intersection_right.len() % 2 == 1)
    {
        retval = true;
    }
    retval
}

/// The below function reads the incoming json file which is of the format
///{
//  "shape": "Polygon",
//  "vehicle": "van",
//  "shape_coordinate":[ {"lat": -2.0,"lon": 3.0},{"lat": 4.0,"lon": 4.0},
//  "moving_coordinate": [{"lat":1.0 ,"lon": 1.0}]
//}
/// shape_coordinate -> contains the coordinates which is used to create the Polygon fence
/// moving_coordinate -> contains series of lat, lon for which location has to be determined.
/// maps it to the MovingTracker struct and returns the result to the caller.
///The function the below parameters
/// filename -> contains the json file which contains the coordinates
/// Returns Result -> contains either MovingTracker struct if success or Error if any failure.

fn read_movingtracker_file(filename: &str) -> Result<MovingTracker, Box<dyn Error>> {
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

fn get_json_info(filename: &str) -> MovingTracker {
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
/// Creates polygon fence based on the coordinates given in the input file
/// Executes set of coordinates for which position has to be determined.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates
///
pub fn execute_polygon(filename: &str, delay: bool) -> String {
    let second = time::Duration::from_millis(1000);
    // let start = get_current_time();
    let u = get_json_info(filename);
    let cor: Vec<Coordinates> = u.shape_coordinate;
    let run: Vec<Coordinates> = u.moving_coordinate;
    //let mut vec_ret:Vec<String> = Vec:new();
    let mut ret_val = "".to_string();
    display_bold(
        "Created Polygon Fence, with the below coordinates",
        Colour::Blue,
    );
    display(&format!("{:?}", cor), Colour::Blue);
    display_bold("Tracking and Verifying, if the below moving objects position is within or outside the fence", Colour::Black);

    for x in run {
        let mut str = "is out of the fence";
        if delay {
            thread::sleep(second);
        }
        if contains(&x, &cor) {
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
    // println!("The time taken for the execute method to perform this operation is {:?}", calculate_run_time(&start));
    ret_val
}

/// The function does the below
/// Reads the input file
/// Creates polygon fence based on the coordinates given in the input file
/// checks if the given coordinates in the function paramater is within the circular fence.
/// Prints if the coordinates are within or outside the fence
///The function the below parameters
/// filename -> contains the json file which contains the coordinates which is necessary to create the fence
/// lat and lon -> search coordinates

pub fn contains_in_polygon(filename: &str, lat: f64, lon: f64) -> bool {
    let u = get_json_info(filename);
    let cor: Vec<Coordinates> = u.shape_coordinate;
    let point: Coordinates = Coordinates::new(lat, lon);
    let mut retval = false;
    // let start = get_current_time();
    display_bold(
        "Searching the vehicle in Polygon Fence, which is built with coordinates",
        Colour::Blue,
    );
    display(&format!("{:?}", cor), Colour::Blue);
    //println!("******** Searching the vehicle in Polygon Fence, which is built with coordinates {:?} *********", cor);
    let mut str = "is out of the fence";
    if contains(&point, &cor) {
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
    //  println!("The time taken for the contain method to perform this operation is {:?}", calculate_run_time(&start));
    retval
}
