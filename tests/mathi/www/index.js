import * as mathi from "mathi";

console.log('mathi', mathi);
const interf = mathi.typedinterface_js();
console.log(interf);

console.log(mathi.sum(1, 5));
console.log(mathi.sub(7, 5));

console.log(mathi.typedinterface_js());

console.log(mathi[interf[0].name](4, 5))
