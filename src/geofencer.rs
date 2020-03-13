// Copyright © 2020 Delhi Durai and Rajeswari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///
/// Geofencer acts as a router, that routes the incoming request to
/// libraries depending on the Shape of the fence
///
use circle::contains_in_circle;
use circle::execute_circle;
use coordinates::Shape;
use polygon::contains_in_polygon;
use polygon::execute_polygon;
use triangle::contains_in_triangle;
use triangle::execute_triangle;

/// Calls the execute method depending on the Shape
pub fn execute(shape: Shape, filename: &str, delay: bool) -> String {
    match shape {
        Shape::Polygon => execute_polygon(filename, delay),
        Shape::Circle => execute_circle(filename, delay),
        Shape::Triangle => execute_triangle(filename, delay),
    }
}

///
/// Calls the contains method depending on the Shape.
///
pub fn contains(shape: Shape, filename: &str, latitude: f64, longitude: f64) -> bool {
    match shape {
        Shape::Polygon => contains_in_polygon(filename, latitude, longitude),
        Shape::Circle => contains_in_circle(filename, latitude, longitude),
        Shape::Triangle => contains_in_triangle(filename, latitude, longitude),
    }
}
