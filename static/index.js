const wasm = import("./wasm_website_frontend");

wasm.then(module => {
  // module.start(document.getElementById('app'));
  console.log('yooo');
  module.greet();
});
