///
/// Test case for testing the geo fence functionality
///

// Test case to validate on point for Polygon
#[test]
fn test_polygon_contains() {
    let filename = "polygon_geofence.json";
    assert!(geofencing::geofencer::contains(
        geofencing::coordinates::Shape::Polygon,
        filename,
        3.0,
        3.0
    ));
    assert_eq!(
        geofencing::geofencer::contains(
            geofencing::coordinates::Shape::Polygon,
            filename,
            5.0,
            5.0
        ),
        false
    );
}
// Test case to validate on point for Polygon with actual Latitude and Longitude
#[test]
fn test_polygon_contains_withactual_latandlong() {
    let filename = "polygon_geofence_2.json";
    assert!(geofencing::geofencer::contains(
        geofencing::coordinates::Shape::Polygon,
        filename,
        55.746768,
        37.625605
    ));
}

// Test case for negative scenario
#[test]
#[should_panic]
fn test_polygon_withincorrect_filename() {
    let filename = "geofence_wrongfile.json";
    assert!(geofencing::geofencer::contains(
        geofencing::coordinates::Shape::Polygon,
        filename,
        55.746768,
        37.625605
    ));

    assert!(geofencing::geofencer::contains(
        geofencing::coordinates::Shape::Circle,
        filename,
        55.746768,
        37.625605
    ));

    assert!(geofencing::geofencer::contains(
        geofencing::coordinates::Shape::Triangle,
        filename,
        55.746768,
        37.625605
    ));
}
// Test case to validate on point for triangle
#[test]
fn test_triangle_contains() {
    let filename = "triangle_geofence.json";
    assert_eq!(
        geofencing::geofencer::contains(
            geofencing::coordinates::Shape::Triangle,
            filename,
            3.0,
            3.0
        ),
        true
    );
    assert_eq!(
        geofencing::geofencer::contains(
            geofencing::coordinates::Shape::Triangle,
            filename,
            5.0,
            7.0
        ),
        false
    );
}

// Test case to validate on point for Circle
#[test]
#[should_panic]
fn test_circle_contains() {
    let filename = "circle_geofence.json";
    assert_eq!(
        geofencing::geofencer::contains(geofencing::coordinates::Shape::Circle, filename, 3.0, 3.0),
        true
    );
    assert_eq!(
        geofencing::geofencer::contains(geofencing::coordinates::Shape::Circle, filename, 7.0, 7.0),
        false
    );
    assert_eq!(
        geofencing::geofencer::contains(geofencing::coordinates::Shape::Circle, filename, 2.0, 1.0),
        false
    );
}
// Negative Test case to validate on point for Circle
#[test]
#[should_panic]
fn test_circle_with_incorrect_input() {
    let filename = "triangle_geofence.json";
    assert_eq!(
        geofencing::geofencer::contains(geofencing::coordinates::Shape::Circle, filename, 3.0, 3.0),
        true
    );
}
// Negative Test case to validate on point for Circle
#[test]
#[should_panic]
fn test_polygon_with_incorrect_input() {
    let filename = "circle_geofence.json";
    assert_eq!(
        geofencing::geofencer::contains(
            geofencing::coordinates::Shape::Polygon,
            filename,
            3.0,
            3.0
        ),
        true
    );
}
// Negative Test case to validate on point for Circle
#[test]
#[should_panic]
fn test_triangle_with_incorrect_input() {
    let filename = "circle_moving_tracker1.json";
    assert_eq!(
        geofencing::geofencer::contains(
            geofencing::coordinates::Shape::Triangle,
            filename,
            3.0,
            3.0
        ),
        true
    );
}

#[test]
fn test_polygon_moving_target() {
    let filename = "polygon_moving_tracker1.json";
    assert_ne!(
        geofencing::geofencer::execute(geofencing::coordinates::Shape::Polygon, filename, false),
        ""
    );
}

#[test]
fn test_circle_moving_target() {
    let filename = "circle_moving_tracker1.json";
    assert_ne!(
        geofencing::geofencer::execute(geofencing::coordinates::Shape::Circle, filename, false),
        ""
    );
}

#[test]
fn test_triangle_moving_target() {
    let filename = "triangle_moving_tracker1.json";
    assert_ne!(
        geofencing::geofencer::execute(geofencing::coordinates::Shape::Triangle, filename, false),
        ""
    );
}
