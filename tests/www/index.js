import * as mathi from "mathi";
import * as lib2 from "lib2";
import * as complex from "complex";

import {dT, showControl} from "@pipeos/dtype-controls";
import {initMenu} from './menu.js';
import {setInputs, setOutputs} from './dtype-controls';

console.log('mathi', mathi);
console.log('lib2', lib2);
console.log('complex', complex);

const modules = {};
const moduleInterfaces = {};

modules.mathi = mathi;
modules.lib2 = lib2;
modules.complex = complex;

moduleInterfaces.mathi = setSignature(mathi.typedinterface_js());
moduleInterfaces.lib2 = setSignature(lib2.typedinterface_js());
moduleInterfaces.complex = setSignature(complex.typedinterface_js());

console.log('moduleInterfaces', moduleInterfaces);

console.log(mathi.sum(1, 5));
console.log(mathi.sub(7, 5));
console.log(mathi[moduleInterfaces.mathi[0].name](4, 5))

initMenu(moduleInterfaces, (moduleName, functioni) => {
  setInputs(functioni, 'controlInputs', (output) => {
    console.log('output', output);
    document.getElementById("controlOutputs").innerHTML = "";
    const args = output.value.map(out => parseFloat(out.toString()));

    // run wasm module
    console.log(moduleName, modules[moduleName], functioni.name)
    const result = modules[moduleName][functioni.name](...args);

    console.log('result', result)
    const outputs = functioni.outputs.map(out => {
      out.value = new dT.BN(result);
      return out;
    })
    console.log('outputs', outputs)
    setOutputs(outputs, "controlOutputs");
  });
});

function setSignature(minterface) {
  return minterface.map(functioni => {
    const ins = functioni.inputs.map(inp => `${inp.label}: ${inp.name}`).join(',');
    const outs = functioni.outputs.map(out => out.name).join(',');
    const returns = outs.length > 0 ? ` -> ${outs}` : '';
    functioni.signature = `${functioni.name}(${ins})${returns}`;
    return functioni;
  });
}
