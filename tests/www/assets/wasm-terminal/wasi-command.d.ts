import { WasmFs } from "@wasmer/wasmfs";
import Command from "./command";
import CommandOptions from "./command-options";
export default class WASICommand extends Command {
    constructor(options: CommandOptions);
    run(wasmFs: WasmFs): Promise<void>;
}
