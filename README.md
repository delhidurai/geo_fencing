##### Final Project Geo Fencing. Class CS 510

##### Name:- Delhi Durai  -  Student ID:-  948639812 - Email ID:- dmoha2@pdx.edu
##### 

##### Name:- Rajeswari  -  Student ID:- 916809371 - Email ID:- rajes2@pdx.edu
#####  
##### 
##### Project Name :- RUST GEO FENCING CRATE

The Geofencing crate is a rust library that uses coordinate geometry to calculate if a given point is within or outside the fence, 
the fence is built using several individual coordinates.  It uses latitude and longitude as coordinate grids to create the fence and to search. 
This Library will provide three fence options
1. Circle
2. Polygon
3. Triangle 

It also provides two search options
1. Real time search:- Search if a moving object is within or outside the fence at different point in time.
2. On Point search:- Search a given coordinate is within or outside the fence.



## Step 1:-
## Create lib.rs and dependent modules
lib.rs and dependent modules that includes the below  
1. circle.rs - Provides methods to create circular fence and to validate if a given point or a continuous cluster of points 
are within or outside the fence. 
2. coordinate.rs - Contains structs and utility function that will be used by the processors.
3. geofencer.rs - Routes the incoming request to respective processor based on the shape requirement. 
4. polygon.rs - Provides methods to create circular fence and to validate if a given point or a continuous cluster of points 
              are within or outside the fence.
5. triangle.rs - Provides methods to create circular fence and to validate if a given point or a continuous cluster of points 
               are within or outside the fence.
6. lib.rs - includes all the above dependent modules.



## Step 2:-
## Create simulator 
#### 1. geo_fence_simulator.rs - 
######Provides command line interface that simulates two different geofencing options.
######Simulation 1:- Real time search
Validate the coordinates of a moving object to find if it is within or outside  the created fence at any given point in time.
######Simulation 2:- Validate particular coordinate
Search a given coordinate is within or outside the fence.

Both the simulation requires coordinates data that defines the fence and (incase of Real time search, coordinates of the moving object) in json format. 

Please refer to the document ##### ReadMe_Execution.pdf ##### before running the simulator

Running the Simulator
Use
```bash
$ cargo run --example geofence_simulator 
```  



## Step 3:-
## Testing  the code
Create test_geofencing.rs that has testcases to unit test the geo fencing crate. 
The test case covers the below scenarios
 1. Happy path unit test case for Circle, Polygon and Triangle shapes.
 2. Negative testing with incorrect file name, for all the Circle, Polygon and Triangle shapes.
 3. Test cases for testing both Intransit objects and a particular coordinate.
Executing test functions
Use
```bash
$ cargo test 
```
If testing under debug mode to view the println's
Use
```bash
$ cargo test -- --nocapture
```

## Step 3:-
## Formatting the code through cargo fmt and cargo clippy
Use 
```bash
$ cargo fmt 
```
to format the code and 

```bash
$ cargo clippy 
```

### Test data
The test data is available as json file in  geofencing/data/ folder. 
The test data with name {shape}_moving_tracker*.json will be used to test simulation 1 - In transit search 
The test data with name {shape}_geofence.json will be used to test simulation 2 - Validate particular coordinate

### What worked?
1. Created geofence solution for three different shapes (Polygon, Circle and Triangle)
2. Successfully validate if a given coordinate is within or outside the created fence.
3. Json file is used as data file that will have the coordinate details for creating the fence and running the simulations. Used serde::json to associate the data file to respective structs.
4. Have used our own customzied json geofence structure that fits our current functionality.
4. Provided user interface for running simulation and viewing the results.

### What didnt work?
1. Have not integrated with Sqlite or any other in memory database, as we felt that Json data file is sufficient for the current functionality.
2. The Polygon fence when created with too many coordinates with rough and irregular borders, sometimes provide incorrect and unpredictable results. This needs to be worked upon.
3. Have not used GeoJSON as it has more information that are not required for the current functionality of this project.
4. Have not tested with real geographic latitude and longitude because of lack of test data.

### How satisfied are you with the result?
1. For the shape Circle and Triangle we have 100% accuracy percentage during our testing.
2. For the shape Polygon we have around 95% accuracy. As discussed before the polygon fence behaves odd for those fences with too many coordinates and has random irregularity in the border.

### What would you like to improve in the future?
1. Revisit the fencing algorithm especially for Polygon and make it more rugged.
2. Planning to expose this as a webservice, implement Sqlite inmemory data store.
3. Provide graphical user interface for visualizing the fence and real time tracking of moving objects.