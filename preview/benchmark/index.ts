import {hello, Matrix, pass_params_cost_test} from "wgpu-wasm-demo";

hello("elase");

let amountData = [];
const cols = 1000;
const rows = 1000;
for (let i = 0; i < cols; i++) {
    let col = [];
    for (let j = 0; j < rows; j++) {
        col.push(j);
    }
    amountData.push(col);
}
// @ts-ignore
const amountDataSize = new Blob([amountData]).size;
let start = Date.now();
pass_params_cost_test(amountData);
console.log(`pass_params_cost_test(${amountDataSize}) cost: ${Date.now() - start}`);
start = Date.now();
const matrix = Matrix.from_array(amountData);
console.log(`create matrix(${amountDataSize}) cost: ${Date.now() - start}`);
start = Date.now();
const matrixList = []
for (let i = 0; i < 1000; i++) {
    matrixList.push(Matrix.from_array([[1, 2, 3], [1, 2, 3], [1, 2, 3]]));
}
console.log(`create multi matrix(${matrixList.length}) cost: ${Date.now() - start}`);
// @ts-ignore
const matrixListSize = new Blob([matrixList]).size;
console.log(`create multi matrix(${matrixListSize})`);