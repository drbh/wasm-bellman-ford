# WASM - Bellman Ford

## Welcome the Bellman Ford implementation in Rust+WASM  

### What is Bellman-Ford?  

The Bellman–Ford algorithm is an algorithm that computes shortest paths from a single source vertex to all of the other vertices in a weighted digraph. It is slower than Dijkstra's algorithm for the same problem, but more versatile, as it is capable of handling graphs in which some of the edge weights are negative numbers. The algorithm was first proposed by Alfonso Shimbel 1955, but is instead named after Richard Bellman and Lester Ford, Jr., who published it in 1958 and 1956, respectively. Edward F. Moore also published the same algorithm in 1957, and for this reason it is also sometimes called the Bellman–Ford–Moore algorithm.  
[wikipedia](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm)

Since we can use Bellman-Ford as an efficient way to detect negative cycles in a fully connected graph, we can use this to quickly find market inefficencies and arbitrage oppertunities. If a cycle is negative in a graph of currency conversions. eg. `USD -> JPY`, `JPY -> EUR` then `EUR -> USD`. Then we can conclude that the currencies are not in price equilibrium.  


Check out [RESOURCES.md](./RESOURCES.md) for a Python example of the code and more resouces about the Bellman-Ford algo.  

Clone the repo and navigate to the `www` dirc, and run the web app. If you need to recompile the wasm then follow the `wasm-pack` instructions below. 
```
git clone https://github.com/drbh/wasm-bellman-ford.git
cd www
npm install
npm run start
```

Open your browser and navigate to:  

```
http://localhost:8080/
```

## Javascripy API

In the example file `index.html` just loads in `index.js` which is the file that interacts with the wasm library. Here is the full code in `index.js` and you can see how we pass a JSON payload into the wasm function.

```
import * as wasm from "wasm-bellman-ford";

var data = JSON.stringify(
{
	"graph": [
	    [1.00, 2.00, 4.00],
	    [0.50, 1.00, 2.00],
	    [0.25, 0.50, 1.00]
	]
})

var bool_response = wasm.bellman_ford_neg_cycle(data);
console.log(bool_response)
```

## rust API
This Bellman Ford algorithim is written in rust and can be used as a component in a larger application or can be editied for other wasm uses. 

Here we see an example function that reads in a JSON formatted string, wich `bellman_ford_neg_cycle` will parse.
```
fn main() {
    let data = r#"
        {
        	"graph": [
			    [1.00, 2.00, 4.00],
			    [0.50, 1.00, 2.00],
			    [0.25, 0.50, 1.00]
			]
        }"#;


    let result = bellman_ford_neg_cycle(data);
    println!("{:?}", result);
}
```


Following this template: [rustwasm/game-of-life](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#putting-it-into-a-web-page)

## Build Pack Instructions
A template for kick starting a Rust and WebAssembly project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
