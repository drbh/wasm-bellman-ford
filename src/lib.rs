mod utils;

use std::f64::INFINITY;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-bellman-ford!");
}

// use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_derive::{Serialize, Deserialize};

use serde_json::error::Error;


#[derive(Serialize, Deserialize)]
struct RootInterface {
  graph: Vec<Vec<f64>>,
}

fn read_json_graph(data: &str) -> Result<(Vec<Vec<f64>>), Error>  {
    let p: RootInterface = serde_json::from_str(data)?;
    Ok(p.graph)
}

#[wasm_bindgen]
pub fn bellman_ford_neg_cycle(data: &str) -> bool {
   //  let data = r#"
   //      {
   //      	"graph": [
			//     [1.00, 2.00, 4.00],
			//     [0.50, 1.00, 2.00],
			//     [0.25, 0.50, 1.00]
			// ]
   //      }"#;

    let table = read_json_graph(data).unwrap();
    // println!("{:?}", table);
    
    let mut transformed_graph = table;
    for row in transformed_graph.iter_mut() {
        for cell in row.iter_mut() {
            let y = -1. * f64::from(*cell).ln();
            *cell = y;
        }
    }
    // Pick any source vertex -- we can run Bellman-Ford from any vertex and
    // get the right result
    let source = 0;
    let n = transformed_graph.len();
    
    // build empty vector
    let mut min_dist: Vec<f64> = Vec::with_capacity(n);
    for _i in 0..n {
        min_dist.push(INFINITY);
    }
    
    min_dist[source] = 0.0;

    // Relax edges |V - 1| times
    for _i in 0..n-1 {
        for v in 0..n {
            for w in 0..n {
                if min_dist[w] > min_dist[v] + transformed_graph[v][w] {
                    min_dist[w] = min_dist[v] + transformed_graph[v][w]
                }
            }
        }
    }
    println!("{:?}", min_dist);
    // If we can still relax edges, then we have a negative cycle
    for v in 0..n {
        for w in 0..n {
            if min_dist[w] > min_dist[v] + transformed_graph[v][w] {
                return true;
            }
        }
    }
    return false;
}

// fn main() {
//     let data = r#"
//         {
//         	"graph": [
// 			    [1.00, 2.00, 4.00],
// 			    [0.50, 1.00, 2.00],
// 			    [0.25, 0.50, 1.00]
// 			]
//         }"#;


//     let result = bellman_ford_neg_cycle(data);
//     println!("{:?}", result);
// }
