async function init() {
     
    // load the wasm file
    const resp = await fetch("sum.wasm");
    const intructions = await resp.arrayBuffer()
    const wasm = await WebAssembly.instantiate(intructions);
    const sumFn = wasm.instance.exports.sum;
    const result = sumFn(250, 300);
    console.log(result);
    alert(result);
}
init();