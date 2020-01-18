import * as mathi from "mathi";
import {initMenu} from './menu.js';

console.log('mathi', mathi);
const interf = mathi.typedinterface_js();
console.log(interf);

console.log(mathi.sum(1, 5));
console.log(mathi.sub(7, 5));
console.log(mathi[interf[0].name](4, 5))


initMenu({mathi: interf});
