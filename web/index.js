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

