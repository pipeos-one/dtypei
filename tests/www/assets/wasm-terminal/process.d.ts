/// <reference types="node" />
import { WasmFs } from "@wasmer/wasmfs";
import CommandOptions from "../command/command-options";
import Command from "../command/command";
export default class Process {
    commandOptions: CommandOptions;
    wasmFs: WasmFs;
    originalWasmFsJson: any;
    dataCallback: Function;
    endCallback: Function;
    errorCallback: Function;
    sharedStdin?: Int32Array;
    startStdinReadCallback?: Function;
    pipedStdin: string;
    stdinPrompt: string;
    readStdinCounter: number;
    command: Command;
    constructor(commandOptions: CommandOptions, wasmFsJson: any, dataCallback: Function, endCallback: Function, errorCallback: Function, sharedStdinBuffer?: SharedArrayBuffer, startStdinReadCallback?: Function);
    start(pipedStdinData?: Uint8Array): Promise<void>;
    stdoutWrite(stdoutBuffer: Buffer | Uint8Array, offset?: number, length?: number, position?: number): number;
    stdinRead(stdinBuffer: Buffer | Uint8Array, offset?: number, length?: number, position?: number): number;
}
