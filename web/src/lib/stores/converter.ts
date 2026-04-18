import { writable, derived } from 'svelte/store';

export type InputFormat = 'json' | 'toml';

export const inputText    = writable<string>('');
export const inputFormat  = writable<InputFormat>('json');
export const threshold    = writable<number>(50);
export const isConverting = writable<boolean>(false);
export const outputText   = writable<string>('');
export const errorMsg     = writable<string>('');

/** Derived store: true when input is non-empty and not whitespace-only. */
export const hasInput = derived(inputText, $t => $t.trim().length > 0);
