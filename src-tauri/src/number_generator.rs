use rand::random;
use serde::Serialize;

// Define a struct to hold the series configuration and data.
// Optional: uncomment derive(Serialize) if needed
#[derive(Serialize, Debug)] // Debug is useful for printing
pub struct SeriesObject {
    // Use Vec<[f64; 2]> for [[x1, y1], [x2, y2], ...] structure
    data: Vec<[f64; 2]>,
    #[serde(rename = "lineWidth")]
    line_width: i16,
    #[serde(rename = "boostThreshold")]
    boost_threshold: i16,
}

// Generates a single series data array
// Equivalent to the TypeScript getData function
pub fn get_data(n: usize) -> Vec<[f64; 2]> {
    let mut arr: Vec<[f64; 2]> = Vec::with_capacity(n); // Pre-allocate capacity
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    let mut spike: f64; // Declared here

    for i in 0..n {
        // Update variables periodically
        if i % 100 == 0 {
            a = 2.0 * random::<f64>(); // random::<f64>() gives a float between 0.0 and 1.0
        }
        if i % 1000 == 0 {
            b = 2.0 * random::<f64>();
        }
        if i % 10000 == 0 {
            c = 2.0 * random::<f64>();
        }
        // Original spike logic seems to always result in 0
        if i % 50000 == 0 {
            spike = 0.0;
        } else {
            spike = 0.0;
        }

        // Calculate the y-value
        // Need to cast 'i' to f64 for calculations
        let y_val = 2.0 * (i as f64 / 100.0).sin() + a + b + c + spike + random::<f64>();

        // Push the [x, y] pair
        // Cast 'i' to f64 for the x-coordinate as well
        arr.push([i as f64, y_val]);
    }
    arr
}

// Generates multiple series objects
// Equivalent to the TypeScript getSeries function
#[tauri::command]
pub fn get_series(n: usize, s: usize) -> Vec<SeriesObject> {
    let mut series_vec: Vec<SeriesObject> = Vec::with_capacity(s);
    for _ in 0..s {
        let data = get_data(n);
        series_vec.push(SeriesObject {
            data,
            line_width: 1,
            boost_threshold: 1,
        });
    }
    return series_vec;
}
