import { writable } from 'svelte/store';

export type WasmStatus = 'idle' | 'loading' | 'ready' | 'error';

export const wasmStatus = writable<WasmStatus>('idle');
export const wasmError  = writable<string>('');

type ConverterWasm = {
  convert_json: (input: string, threshold: number) => string;
  convert_toml: (input: string, threshold: number) => string;
  version: () => string;
};

let _wasm: ConverterWasm | null = null;

/** Load the WASM module once and cache it. */
export async function loadWasm(): Promise<ConverterWasm> {
  if (_wasm) return _wasm;
  wasmStatus.set('loading');
  try {
    // Dynamic import — resolved by Vite after wasm-pack build
    const module = await import('$lib/wasm/converter_wasm.js');
    await module.default(); // calls wasm init()
    _wasm = module as unknown as ConverterWasm;
    wasmStatus.set('ready');
    return _wasm;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    wasmError.set(msg);
    wasmStatus.set('error');
    throw err;
  }
}
