/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Direction {
  UP = 0,
  DOWN = 1,
  LEFT = 2,
  RIGHT = 3,
}
/**
*/
export class Coord {
  free(): void;
/**
*/
  x: number;
/**
*/
  y: number;
}
/**
*/
export class Food {
  free(): void;
/**
*/
  readonly get_color: string;
/**
*/
  x: number;
/**
*/
  y: number;
}
/**
*/
export class Game {
  free(): void;
/**
*/
  constructor();
/**
*/
  play(): void;
/**
*/
  is_running: boolean;
/**
*/
  score: number;
}
/**
*/
export class Node {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @param {Direction} direction
*/
  constructor(x: number, y: number, direction: Direction);
/**
* @param {Direction} direction
* @returns {Node}
*/
  static random(direction: Direction): Node;
/**
*/
  coord: Coord;
/**
*/
  direction: Direction;
}
/**
*/
export class Snake {
  free(): void;
/**
*/
  constructor();
/**
*/
  grow_snake(): void;
/**
* @returns {(Node)[]}
*/
  get_snake_body(): (Node)[];
/**
*/
  size: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number) => void;
  readonly __wbg_get_game_is_running: (a: number) => number;
  readonly __wbg_set_game_is_running: (a: number, b: number) => void;
  readonly game_new: () => number;
  readonly game_play: (a: number) => void;
  readonly __wbg_food_free: (a: number) => void;
  readonly __wbg_get_food_x: (a: number) => number;
  readonly __wbg_set_food_x: (a: number, b: number) => void;
  readonly __wbg_get_food_y: (a: number) => number;
  readonly __wbg_set_food_y: (a: number, b: number) => void;
  readonly food_get_color: (a: number, b: number) => void;
  readonly __wbg_set_game_score: (a: number, b: number) => void;
  readonly __wbg_get_game_score: (a: number) => number;
  readonly __wbg_coord_free: (a: number) => void;
  readonly __wbg_get_coord_x: (a: number) => number;
  readonly __wbg_set_coord_x: (a: number, b: number) => void;
  readonly __wbg_get_coord_y: (a: number) => number;
  readonly __wbg_set_coord_y: (a: number, b: number) => void;
  readonly __wbg_node_free: (a: number) => void;
  readonly __wbg_get_node_coord: (a: number) => number;
  readonly __wbg_set_node_coord: (a: number, b: number) => void;
  readonly __wbg_get_node_direction: (a: number) => number;
  readonly __wbg_set_node_direction: (a: number, b: number) => void;
  readonly node_new: (a: number, b: number, c: number) => number;
  readonly node_random: (a: number) => number;
  readonly __wbg_snake_free: (a: number) => void;
  readonly __wbg_get_snake_size: (a: number) => number;
  readonly __wbg_set_snake_size: (a: number, b: number) => void;
  readonly snake_new: () => number;
  readonly snake_grow_snake: (a: number) => void;
  readonly snake_get_snake_body: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly wasm_bindgen__convert__closures__invoke0_mut__h2d831516dc2e7947: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
