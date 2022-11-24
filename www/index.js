async function init() {
    
    const importObject = {
        myconsole: {
            log: () => {
                console.log("logging...")
            },
            error: () => {
                console.log("error...")
            }
        }
    }
    // load the wasm file
    const resp = await fetch("sum_with_logging.wasm");
    const intructions = await resp.arrayBuffer()
    const wasm = await WebAssembly.instantiate(intructions, importObject);
    debugger
    const sumFn = wasm.instance.exports.sum;
    const result = sumFn(250, 300);
    console.log(result);
    alert(result);
}
init();