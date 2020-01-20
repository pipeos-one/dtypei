import {dT as dtype, showControl, guiRegister} from "@pipeos/dtype-controls";

let dT = dTExtend(dtype);

function setInputs(functioni, domId, callback) {
  let inputs = {value: [], type: {}, name: 'inputs'}

  functioni.inputs.forEach(inp => {
    inputs.type[inp.label] = inp.name;
    inputs.value.push(null);
  });

  inputs = dT.t.apply(inputs, "random")
  inputs.name = 'inputs';

  const inputControls = showControl(inputs, domId, {
    // randomize: true,
    gui: {align: 'left'},
    onChange: (typed) => {
      // console.log('controlInputs onChange', typed);
      inputs = typed;
      inputs.type = inputs.type;
    },
    buttons: [
      {type: "button", label: functioni.signature, action: async () => {
        console.log('-- run', inputs);
        callback(inputs);
      }}
    ]
  });
}

function setOutputs(result, domId) {
  const guiOptions = {gui: {align: 'right'}, buttons: [
    {type: "button", label: "copy", action: async () => {
      console.log('copy');
    }}
  ]};

  if (!result) return;
  const outputs = {value: [], type: {}, name: 'outputs'}

  result.forEach((inp, i) => {
    outputs.type[inp.label] = inp.name;
    outputs.value.push(inp.value);
  });
  showControl(outputs, domId, guiOptions);
}

function dTExtend(dT) {
  // dT.controls["i32"] = {
  //   min: function(min){if (min !== undefined) this._min = min; return this._min;},
  //   max: function(max){if (max !== undefined) this._max = max; return this._max;},
  //   random: function(typed){
  //     // return ({value: 0n, type: "i32"});
  //     return ({value: 3, type: "i32"});
  //   }
  //   // random: function(typed){
  //   //   let type = typed.type
  //   //   let range = dT.t.getRange(typed)
  //   //   let val = range.max.sub(new dT.BN(Math.random()*Number.MAX_SAFE_INTEGER).mul(range.max.sub(range.min)).div(new dT.BN(Number.MAX_SAFE_INTEGER)))
  //   //   return ({value:  val, type: type})
  //   // },
  // }

  dT.controls["u8"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(0),
    max: () => new dT.BN(255),
  })

  dT.controls["i8"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(-128),
    max: () => new dT.BN(127),
  })

  dT.controls["u16"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(0),
    max: () => new dT.BN(65535),
  })

  dT.controls["i16"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(-32768),
    max: () => new dT.BN(32767),
  })

  dT.controls["i32"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(-2147483648),
    max: () => new dT.BN(2147483647),
  })

  dT.controls["u32"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(0),
    max: () => new dT.BN(4294967295),
  })

  dT.controls["i64"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(-9223372036854775808),
    max: () => new dT.BN(9223372036854775807),
  })

  dT.controls["u64"] = Object.assign({}, dT.controls["int32"], {
    min: () => new dT.BN(0),
    max: () => new dT.BN(18446744073709551615),
  })

  dT.controls["f64"] = dT.controls["i64"]
  dT.controls["f32"] = dT.controls["i32"]

  // dT.controls["u32"] = dT.controls["i32"]
  // dT.controls["i64"] = dT.controls["i32"]
  // dT.controls["u64"] = dT.controls["i64"]
  // dT.controls["f64"] = dT.controls["i64"]
  // dT.controls["f32"] = dT.controls["i32"]

  // dT.t.biType  = function (name, min, max){
  //   let test = x => x >= min &&
  //       x <= max
  //   let supertypes = []
  //   dT.controls[name] = Object.assign({},dT.controls["i32"]) // ,dT.controls["bn"]
  //   dT.controls[name].min(min)
  //   dT.controls[name].max(max)
  //   return dT.t.setType(name, test, supertypes)
  // }
  //
  // dT.t.biType("i32", -(BigInt(2) ** BigInt(32-1)-BigInt(1)), BigInt(2) ** BigInt(32-1)-BigInt(1))
  // dT.t.biType("u32", BigInt(0), BigInt(2) ** BigInt(32)-BigInt(1))
  // dT.t.biType("i64", -(BigInt(2) ** BigInt(64-1)-BigInt(1)), BigInt(2) ** BigInt(64-1)-BigInt(1))
  // dT.t.biType("u64", BigInt(0), BigInt(2) ** BigInt(64)-BigInt(1))

  // dT.controls.u32.showControl = function(typed, folder, options = {}) {
  //   const {onChange, args} = options;
  //   const [gui] = args;
  //   const guiOptions = {
  //     type: 'range', label: typed.name+":"+typed.type,
  //     min: dT.controls[typed.type].min().toString(10),
  //     max: dT.controls[typed.type].max().toString(10), step: 1 ,
  //     folder: folder,
  //     initial: typed.value.toString(10),
  //     onChange: (data) => {
  //       typed.value  = BigInt(data)
  //       if (onChange) onChange(typed);
  //    }
  //   }
  //
  //   const component = guiRegister(typed, gui, guiOptions, options);
  //   console.log('component dT.controls["bn"]', component);
  //
  //   return typed
  // }
  //
  //   dT.controls.i32.showControl = function(typed, folder, options = {}) {
  //   const {onChange, args} = options;
  //   const [gui] = args;
  //   const guiOptions = {
  //     type: 'text', label: typed.name+":"+typed.type,
  //     folder: folder,
  //     initial: typed.value,
  //     onChange: (data) => {
  //       console.log(data);
  //       typed.value  = BigInt(data)
  //       console.log(typed);
  //       if (onChange) onChange(typed);
  //     }
  //   }
  //
  //   const component = guiRegister(typed, gui, guiOptions, options);
  //   console.log('dT.controls["string"]', component);
  //
  //   return typed
  // }
  //
  // dT.controls.i64.showControl = dT.controls.i32.showControl
  // dT.controls.u64.showControl = dT.controls.u32.showControl
  //
  // dT.controls.f32.showControl =  function(typed, folder, options = {}) {
  //   const {onChange, args} = options;
  //   const [gui] = args;
  //   const guiOptions = {
  //     type: 'text', label: typed.name+":"+typed.type,
  //     folder: folder,
  //     initial: typed.value,
  //     onChange: (data) => {
  //       console.log(data);
  //       typed.value  = Float32Array([data]);
  //       console.log(typed);
  //       if (onChange) onChange(typed);
  //     }
  //   }
  //
  //   const component = guiRegister(typed, gui, guiOptions, options);
  //   console.log('dT.controls["string"]', component);
  //
  //   return typed
  // }
  //
  // dT.controls.f64.showControl = dT.controls.f32.showControl


  dT.t.extendTypes();
  return dT;
}



export {setInputs, setOutputs};
