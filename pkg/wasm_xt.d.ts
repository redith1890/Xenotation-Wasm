/* tslint:disable */
/* eslint-disable */
/**
* @param {number} n
* @returns {boolean}
*/
export function is_power_of_two(n: number): boolean;
/**
* @param {number} n
* @returns {string}
*/
export function represent_power_of_two(n: number): string;
/**
* @param {number} n
* @returns {Uint32Array}
*/
export function find_primes_up_to(n: number): Uint32Array;
/**
* @param {number} prime
* @param {Uint32Array} primes
* @returns {string}
*/
export function represent_prime(prime: number, primes: Uint32Array): string;
/**
* @param {number} n
* @param {Uint32Array} primes
* @returns {string}
*/
export function represent_number(n: number, primes: Uint32Array): string;
/**
* @param {number} n
* @returns {Uint32Array}
*/
export function descompose_into_primes(n: number): Uint32Array;
/**
* @param {number} number
* @returns {string}
*/
export function representation(number: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly is_power_of_two: (a: number) => number;
  readonly represent_power_of_two: (a: number, b: number) => void;
  readonly find_primes_up_to: (a: number) => number;
  readonly represent_prime: (a: number, b: number, c: number) => void;
  readonly represent_number: (a: number, b: number, c: number) => void;
  readonly descompose_into_primes: (a: number, b: number) => void;
  readonly representation: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
