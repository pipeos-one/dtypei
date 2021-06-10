import { WasmFs } from "@wasmer/wasmfs";
import CommandOptions from "./command-options";
export default class Command {
    options: CommandOptions;
    constructor(options: CommandOptions);
    run(wasmFs: WasmFs): Promise<void>;
}
