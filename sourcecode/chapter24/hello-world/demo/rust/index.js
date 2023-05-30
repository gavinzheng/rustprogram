// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit from "./pkg/hello_world.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await wasmInit("./pkg/hello_world_bg.wasm");

  // Call the Mul function export from wasm, save the result
  const mulResult = helloWorld.mul(7, 8);

  // Set the result onto the body
  document.body.textContent = `Hello Milly! Birth: ${mulResult}`;
};
runWasm();
