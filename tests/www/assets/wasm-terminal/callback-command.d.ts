import Command from "./command";
import CommandOptions from "./command-options";
import { WasmFs } from "@wasmer/wasmfs";
export default class CallbackCommand extends Command {
    callback: Function;
    stdoutCallback?: Function;
    constructor(options: CommandOptions);
    run(wasmFs: WasmFs): Promise<void>;
}
