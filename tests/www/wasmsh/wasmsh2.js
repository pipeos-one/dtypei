import WasmTerminal, { fetchCommandFromWAPM } from "@wasmer/wasm-terminal";

import fetchCommand, { wapm } from "./functions/fetch-command";

const getWelcomeMessage = `WebAssembly.sh v0.0.2
Powered by Wasmer-JS
@wasmer/wasm-terminal v0.6.0

`

function wasmsh(domid, loadCallback) {

  // Let's write handler for the fetchCommand property of the WasmTerminal Config.
  const fetchCommandHandler = async (commandName, commandArgs, envEntries) => {
    console.log('fetchCommandHandler', commandName, commandArgs, envEntries);
    // Let's return a "CallbackCommand" if our command matches a special name
    if (commandName === "callback-command") {
      const callbackCommand = async (options, wasmFs) => {
        console.log('callbackCommand', options, wasmFs);
        return `Callback Command Working! Options: ${options}, fs: ${wasmFs}`;
      };
      return callbackCommand;
    }

    if (commandName === 'load') {
      const wasmBinary = loadCallback(commandArgs[0]);
      return await lowerI64Imports(wasmBinary);
    }

    // if (commandName === 'run') {
    //
    // }

    // Let's fetch a wasm Binary from WAPM for the command name.
    const wasmBinary = await fetchCommandFromWAPM(commandName);

    // lower i64 imports from Wasi Modules, so that most Wasi modules
    // Can run in a Javascript context.
    return await lowerI64Imports(wasmBinary);
  };

  // // Let's create our Wasm Terminal
  // const wasmTerminal = new WasmTerminal({
  //   // Function that is run whenever a command is fetched
  //   fetchCommand: fetchCommandHandler
  // });

  const wasmTerminal = new WasmTerminal({
    // processWorkerUrl: "/assets/wasm-terminal/process.worker.js",
    fetchCommand: fetchCommand,
    wasmFs: wapm.wasmFs,
  });

  // Let's print out our initial message
  wasmTerminal.print(getWelcomeMessage);

  // Let's bind our Wasm terminal to it's container
  const containerElement = document.querySelector("#" + domid);
  wasmTerminal.open(containerElement);
  wasmTerminal.fit();
  // wasmTerminal.focus();

  // Later, when we are done with the terminal, let's destroy it
  // wasmTerminal.destroy();

  return wasmTerminal;
}

export default wasmsh;
