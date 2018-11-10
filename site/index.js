const js = import("./node_modules/rasm/rasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
