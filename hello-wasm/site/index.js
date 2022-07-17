// const js = import("./node_modules/@from20020516/hello-wasm/hello_wasm.js");
const js = import("../pkg/hello_wasm");
js.then(js => {
    js.greet("WebAssembly");
});
