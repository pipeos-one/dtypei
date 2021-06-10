import * as mathi from "mathi";
import * as lib2 from "lib2";
import * as complex from "complex";
import * as dtypei_wrapper from "dtypei-wrapper";

import {dT} from "@pipeos/dtype-controls";
import {initMenu, initModuleMenu, initMenuBehaviour} from './menu.js';
import {setInputs, setOutputs} from './dtype-controls';
import wasmsh from './wasmsh/wasmsh2';


let REQ_INSTALLED = false;
let MOD_INSTALLED = {};
const ICON_MAP = {
  fn: 'functions',
  enum: 'format_list_bulleted',
  struct: 'view_list',
}
const MODI_MAP = {};

const modules = {};
const moduleInterfaces = {};
const moduleUrls = {};
let tempResult = {};
let wasmTerminal;

console.log('mathi', mathi);
console.log('lib2', lib2);
console.log('complex', complex);
console.log(mathi.sum(1, 5));
console.log(mathi.sub(7, 5));
// console.log(mathi[moduleInterfaces.mathi[0].name](4, 5))
// console.log(complex.polish({private_id: 1, legs: 4, color: 1, material: 1, finish: {sanded: false, varnished: false, polished: false}}))

init([
  {name: 'mathi', instance: mathi},
  {name: 'lib2', instance: lib2},
  {name: 'complex', instance: complex},
]);
wasmTerminal = wasmsh('wasmsh', (moduleName) => modules[moduleName]);

let someinterf = JSON.stringify([
  {name: 'sum_i32', type_choice: 'fn', inputs: [{label: 'a', name: 'i32', dimensions: []},{label: 'b', name: 'i32', dimensions: []}], outputs: [{label: 'c', name: 'i32', dimensions: []}]},
  {name: 'sum_u16', type_choice: 'fn', inputs: [{label: 'a', name: 'u16', dimensions: []},{label: 'b', name: 'u16', dimensions: []}], outputs: [{label: 'c', name: 'u16', dimensions: []}]},
]);
// // doit('http://192.168.1.140:6600/sums/pkg/sums_bg.wasm', someinterf)

// https://registry-cdn.wapm.io/contents/petersalomonsen/wasmsong/1.0.0/wasmsong.wasm
document.getElementById('wasmName').value = 'sums';
document.getElementById('wasmUrl').value = 'http://192.168.1.140:6600/sums/pkg/sums_bg.wasm';
document.getElementById('wasmInterface').value = someinterf;

document.getElementById('loadWrappedWasm').onclick = async () => {
  const wasmName = document.getElementById('wasmName').value;
  const wasmUrl = document.getElementById('wasmUrl').value;
  const wasmInterface = document.getElementById('wasmInterface').value;
  if (!wasmName || !wasmUrl || !wasmInterface) {
    console.error('Missing name, url or interface');
    return;
  }
  let parsedInterface;
  try {
    parsedInterface = JSON.parse(wasmInterface);
  } catch (e) {
    throw new Error(e);
  }
  await dtypei_wrapper.init(wasmUrl, wasmInterface);

  moduleUrls[wasmName] = wasmUrl.substring(0, wasmUrl.lastIndexOf('/'));
  init([{name: wasmName, instance: instanceWrap(wasmUrl, parsedInterface)}], false);
}

function instanceWrap(wasmUrl, wasmInterface) {
  const instance = {}
  instance.typedinterface_js = () => wasmInterface;
  wasmInterface.forEach(item => {
    if (item.type_choice !== 'fn') return;
    instance[item.name] = (...inputs) => {
      return dtypei_wrapper.run(wasmUrl, item.name, JSON.stringify(inputs));
    }
  });
  return instance;
}

// async function doit(url, interf) {
//   const initialized = await dtypei_wrapper.init(url, interf);
//   console.log('wrapper initialized', initialized);
//
//   // const result = await dtypei_wrapper.run('http://192.168.1.140:6600/sums/pkg/sums_bg.wasm', 'sum_i32', JSON.stringify([2, 3]));
//   //
//   // console.log('wrapper result', result);
// }


function init(instances, initialize = true) {
  instances.forEach(inst => prepInstance(inst));
  console.log('moduleInterfaces', moduleInterfaces);

  const callback = (moduleName, functioni) => {
    document.getElementById("controlOutputs").innerHTML = "";
    setInputs(
      functioni,
      'controlInputs',
      controlsCallback(moduleName, functioni, runWasmCommand),
      controlsCallback(moduleName, functioni, runWasmCommandTerminal),
      moduleInterfaces[moduleName],
      tempResult.gui,
    );
    // tempResult = {};
  };

  if (initialize) {
    initMenu(moduleInterfaces, ICON_MAP, callback);
  } else {
    instances.forEach(inst => initModuleMenu(inst.name, moduleInterfaces[inst.name], ICON_MAP, callback));
  }
}

function prepInstance(inst) {
  modules[inst.name] = inst.instance;
  moduleInterfaces[inst.name] = setSignature(inst.instance.typedinterface_js());
  MODI_MAP[inst.name] = {};
  moduleInterfaces[inst.name].forEach(finterf => {
    if (finterf.type_choice !== 'fn') {
      MODI_MAP[inst.name][finterf.name] = finterf;
    }
  });
}


// wasmTerminal.runCommand(commandString: string)
// wapm install vshymanskyy/wasm3@0.4.2
// curl https://raw.githubusercontent.com/wasm3/wasm3/master/test/lang/fib32.wasm -o /fib32.wasm

// curl http://localhost:6500/mathi/pkg/mathi_bg.wasm -o /mathi_bg.wasm
// curl http://localhost:6500/mathi/pkg/mathi.js -o /mathi.js

function runWasmCommand(moduleName, functionName, args) {
  return modules[moduleName][functionName](...args);
}

async function runWasmCommandTerminal(moduleName, functionName, args) {
  if (!REQ_INSTALLED) {
    const installWasm3 = `wapm install vshymanskyy/wasm3@0.4.2`;
    await runCommandWait(installWasm3)(1000);
    REQ_INSTALLED = true;
  }

  if (!MOD_INSTALLED[moduleName]) {
    const url = moduleUrls[moduleName] || `http://localhost:6500/${moduleName}/pkg`;
    const loadModuleWasm = `curl ${url}/${moduleName}_bg.wasm -o /${moduleName}_bg.wasm`;
    const loadModuleJs = `curl ${url}/${moduleName}.js -o /${moduleName}.js`;

    await runCommandWait(loadModuleWasm)(1000);
    await runCommandWait(loadModuleJs)(1000);
    MOD_INSTALLED[moduleName] = true;
  }

  const runCommandString = `wasm3 --func ${functionName} /${moduleName}_bg.wasm ${args.join(' ')}`;
  await runCommandWait(runCommandString)(1000);

  return null;
}

const runCommandWait = command => ms => {
  return new Promise((resolve, reject) => {
    wasmTerminal.runCommand(command);
    setTimeout(() => {
      resolve();
    }, ms)
  });
}

// const parseValue = (value) => {
//   if (typeof value === 'string') return value;
//   if (value instanceof Array) {
//     return value.map(parseValue);
//   }
//   return parseFloat(value.toString());
// }

const parseValue = (typed, interf) => {
  const value = typed.value
  console.log('--- parseValue', typed, typeof value);
  if (typeof value === 'string') return value;
  if (value instanceof dT.BN) {
    return parseFloat(value.toString());
  }
  if (value instanceof Array && value.length > 0) {
    const newvalue = {};
    const keys = Object.keys(typed.type);
    // return value.map((item, i) => {
    value.forEach((item, i) => {
      console.log('parseValue', item, i, keys[i]);
      let type = typed.type[keys[i]];
      if (interf[type]) {
        let newtype = {};
        interf[type].inputs.forEach(inp => {
          newtype[inp.label] = inp.name;
        });
        type = newtype;
      }
      console.log('type', type);
      // return parseValue({value: item, type}, interf);
      newvalue[type.label || keys[i]] = parseValue({value: item, type}, interf);
    });
    return newvalue;
  }
  return typed.value;
}

const prepValue = (value, valuei, interf) => {
  if (typeof value === 'string') {
    if (parseFloat(value)) return new dT.BN(value);
    return value;
  }
  if (value instanceof Array) {
    return value.map(prepValue);
  }
  if (typeof value === 'number' || typeof value === 'bigint') {
    return new dT.BN(value);
  }
  if (value instanceof Object) {
    return interf[valuei.name].inputs.map(inp => {
      return prepValue(value[inp.label], inp, interf);
    });
  }
  return value;
}

const controlsCallback = (moduleName, functioni, callback) => async (output) => {
  console.log('output', output);
  // const args = output.value.map(parseValue);
  // const args = output.value.map(out => {
  //   console.log('controlsCallback', out, moduleName, MODI_MAP[moduleName])
  //   return parseValue(out, MODI_MAP[moduleName]);
  // });
  let args = parseValue(output, MODI_MAP[moduleName]);
  console.log('args', args);
  if (args instanceof Object) {
    args = Object.values(args);
    if (tempResult.wasm) {
      args = [tempResult.wasm];
    }
  }
  console.log('args', args);
  // run wasm module
  const result = await callback(moduleName, functioni.name, args);
  if (!result) return;

  console.log('result', result)
  const outputs = functioni.outputs.map(out => {
    out.value = prepValue(result, out, MODI_MAP[moduleName]);
    return out;
  })
  console.log('outputs', outputs)
  // document.getElementById("controlOutputs").innerHTML = "";
  setOutputs(outputs, "controlOutputs", (out) => {
    tempResult = {
      gui: out,
      wasm: result
    }
  });
  tempResult = {};
}

function setSignature(minterface) {
  return minterface.map(functioni => {
    const ins = functioni.inputs.map(inp => `${inp.label}: ${inp.name}`).join(',');
    const outs = functioni.outputs.map(out => out.name).join(',');
    const returns = outs.length > 0 ? ` -> ${outs}` : '';
    functioni.signature = `${functioni.name}(${ins})${returns}`;
    return functioni;
  });
}
