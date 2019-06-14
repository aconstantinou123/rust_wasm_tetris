import * as wasm from './tetris_bg';

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
*/
export const Position = Object.freeze({ Up:0,Right:1,Down:2,Left:3, });
/**
*/
export const ShapeType = Object.freeze({ Bar:0,Square:1,Hat:2,Leg:3,ReverseLeg:4,Step:5,ReverseStep:6, });

export function __wbg_error_4bb6c2a97407129a(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);

    console.error(varg0);
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

export function __wbg_new_59cb74e423758ede() {
    return addHeapObject(new Error());
}

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

export function __wbg_stack_558ba5917b466edd(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

function freeTetrisBoard(ptr) {

    wasm.__wbg_tetrisboard_free(ptr);
}
/**
*/
export class TetrisBoard {

    static __wrap(ptr) {
        const obj = Object.create(TetrisBoard.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeTetrisBoard(ptr);
    }

    /**
    * @returns {TetrisBoard}
    */
    static new() {
        return TetrisBoard.__wrap(wasm.tetrisboard_new());
    }
    /**
    * @returns {number}
    */
    get_width() {
        return wasm.tetrisboard_get_width(this.ptr) >>> 0;
    }
    /**
    * @returns {number}
    */
    get_score() {
        return wasm.tetrisboard_get_score(this.ptr) >>> 0;
    }
    /**
    * @returns {boolean}
    */
    get_game_over() {
        return (wasm.tetrisboard_get_game_over(this.ptr)) !== 0;
    }
    /**
    * @returns {void}
    */
    set_game_over() {
        return wasm.tetrisboard_set_game_over(this.ptr);
    }
    /**
    * @returns {boolean}
    */
    get_generate_new_shape() {
        return (wasm.tetrisboard_get_generate_new_shape(this.ptr)) !== 0;
    }
    /**
    * @returns {void}
    */
    set_generate_new_shape() {
        return wasm.tetrisboard_set_generate_new_shape(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_height() {
        return wasm.tetrisboard_get_height(this.ptr) >>> 0;
    }
    /**
    * @returns {number}
    */
    get_space_used() {
        return wasm.tetrisboard_get_space_used(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_space_length() {
        return wasm.tetrisboard_get_space_length(this.ptr) >>> 0;
    }
    /**
    * @param {Tetromino} tetronimo_clone
    * @returns {void}
    */
    check_shape_is_out_of_bounds(tetronimo_clone) {
        return wasm.tetrisboard_check_shape_is_out_of_bounds(this.ptr, tetronimo_clone.ptr);
    }
    /**
    * @param {Tetromino} tetronimo_clone
    * @returns {number}
    */
    check_collision(tetronimo_clone) {
        return wasm.tetrisboard_check_collision(this.ptr, tetronimo_clone.ptr) >>> 0;
    }
    /**
    * @returns {void}
    */
    check_for_completed_row() {
        return wasm.tetrisboard_check_for_completed_row(this.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    add_shape_to_space_used(tetronimo) {
        return wasm.tetrisboard_add_shape_to_space_used(this.ptr, tetronimo.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    get_shape_position(tetronimo) {
        return wasm.tetrisboard_get_shape_position(this.ptr, tetronimo.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    update_shape_position(tetronimo) {
        return wasm.tetrisboard_update_shape_position(this.ptr, tetronimo.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    move_shape_down(tetronimo) {
        return wasm.tetrisboard_move_shape_down(this.ptr, tetronimo.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    move_shape_right(tetronimo) {
        return wasm.tetrisboard_move_shape_right(this.ptr, tetronimo.ptr);
    }
    /**
    * @param {Tetromino} tetronimo
    * @returns {void}
    */
    move_shape_left(tetronimo) {
        return wasm.tetrisboard_move_shape_left(this.ptr, tetronimo.ptr);
    }
    /**
    * @returns {string}
    */
    render() {
        const retptr = globalArgumentPtr();
        wasm.tetrisboard_render(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;

    }
}

function freeTetromino(ptr) {

    wasm.__wbg_tetromino_free(ptr);
}
/**
*/
export class Tetromino {

    static __wrap(ptr) {
        const obj = Object.create(Tetromino.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeTetromino(ptr);
    }

    /**
    * @param {number} rand
    * @returns {Tetromino}
    */
    static generate_random_shape(rand) {
        return Tetromino.__wrap(wasm.tetromino_generate_random_shape(rand));
    }
    /**
    * @returns {Tetromino}
    */
    static new_reverse_step_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_reverse_step_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_step_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_step_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_leg_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_leg_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_reverse_leg_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_reverse_leg_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_square_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_square_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_bar_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_bar_shape());
    }
    /**
    * @returns {Tetromino}
    */
    static new_hat_shape() {
        return Tetromino.__wrap(wasm.tetromino_new_hat_shape());
    }
    /**
    * @returns {number}
    */
    get_block_size() {
        return wasm.tetromino_get_block_size(this.ptr) >>> 0;
    }
    /**
    * @returns {number}
    */
    get_block1() {
        return wasm.tetromino_get_block1(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_block2() {
        return wasm.tetromino_get_block2(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_block3() {
        return wasm.tetromino_get_block3(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_block4() {
        return wasm.tetromino_get_block4(this.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    move_shape_down(board) {
        return wasm.tetromino_move_shape_down(this.ptr, board.ptr);
    }
    /**
    * @returns {void}
    */
    move_shape_right() {
        return wasm.tetromino_move_shape_right(this.ptr);
    }
    /**
    * @returns {void}
    */
    move_shape_left() {
        return wasm.tetromino_move_shape_left(this.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_shape(board) {
        return wasm.tetromino_transform_shape(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_bar(board) {
        return wasm.tetromino_transform_bar(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_step(board) {
        return wasm.tetromino_transform_step(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_reverse_step(board) {
        return wasm.tetromino_transform_reverse_step(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_hat(board) {
        return wasm.tetromino_transform_hat(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_leg(board) {
        return wasm.tetromino_transform_leg(this.ptr, board.ptr);
    }
    /**
    * @param {TetrisBoard} board
    * @returns {void}
    */
    transform_reverse_leg(board) {
        return wasm.tetromino_transform_reverse_leg(this.ptr, board.ptr);
    }
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

