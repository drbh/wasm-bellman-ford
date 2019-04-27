# Resources

This project was an attempt at porting a simple algorithm from `Python` into a safer/faster language `rust` with the goal of leveraging the Rust+Wasm ecosystem to provide a super fast way to execute Bellman Ford on a graph adjacency table. 

### How To Find Arbitrage Opportunities In Python
The blog post by "Daily Coding Problems" shows a consise elegent implementation of the Bellman Ford algorithm [here](https://dailycodingproblem.com/blog/how-to-find-arbitrage-opportunities-in-python/)

### Mathamatical Resources

[http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/FOREX.pdf](http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/FOREX.pdf)  
[https://etrain.github.io/finance/2013/06/08/currency-arbitrage-in-python](https://etrain.github.io/finance/2013/06/08/currency-arbitrage-in-python)  
[https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm)  [https://www.coursera.org/lecture/algorithms-on-graphs/currency-exchange-reduction-to-shortest-paths-cw8Tm](https://www.coursera.org/lecture/algorithms-on-graphs/currency-exchange-reduction-to-shortest-paths-cw8Tm)
[https://courses.csail.mit.edu/6.046/spring04/handouts/ps7sol.pdf](https://courses.csail.mit.edu/6.046/spring04/handouts/ps7sol.pdf)  

The full code below:  

```python
from math import log

def arbitrage(table):
    transformed_graph = [[-log(edge) for edge in row] for row in graph]

    # Pick any source vertex -- we can run Bellman-Ford from any vertex and
    # get the right result
    source = 0
    n = len(transformed_graph)
    min_dist = [float('inf')] * n

    min_dist[source] = 0

    # Relax edges |V - 1| times
    for i in range(n - 1):
        for v in range(n):
            for w in range(n):
                if min_dist[w] > min_dist[v] + transformed_graph[v][w]:
                    min_dist[w] = min_dist[v] + transformed_graph[v][w]

    # If we can still relax edges, then we have a negative cycle
    for v in range(n):
        for w in range(n):
            if min_dist[w] > min_dist[v] + transformed_graph[v][w]:
                return True

    return False
```

The acutal rust implementation looks almost identical:  

```rust
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
```