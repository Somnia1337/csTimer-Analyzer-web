let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

/**
 * Provides information about the dataset and parsed options.
 * @returns {any}
 */
export function analysis_info() {
    const ret = wasm.analysis_info();
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Provides the analysis for the session specified by JS.
 * @param {number} n
 * @returns {any}
 */
export function analyze_nth_session(n) {
    const ret = wasm.analyze_nth_session(n);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Provides number of sessions to be analyzed.
 * @returns {number}
 */
export function get_session_count() {
    const ret = wasm.get_session_count();
    return ret >>> 0;
}

/**
 * Provides debug information about analysis timings.
 * @returns {any}
 */
export function get_timings() {
    const ret = wasm.get_timings();
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Initializes the analysis, parses options and sessions.
 * @param {Uint8Array} options_txt
 * @param {Uint8Array} data_txt
 * @param {HTMLCanvasElement} canvas
 * @param {string} locale
 */
export function init_analysis(options_txt, data_txt, canvas, locale) {
    const ptr0 = passArray8ToWasm0(options_txt, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data_txt, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.init_analysis(ptr0, len0, ptr1, len1, canvas, ptr2, len2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * Converts the markdown content to HTML, a
 * more time-efficient equivalent to marked.js.
 * @param {string} input
 * @returns {any}
 */
export function render_markdown(input) {
    const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.render_markdown(ptr0, len0);
    return ret;
}

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_debug_string_adfb662ae34724b6 = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_string_get_a2a31e16edf96e42 = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_append_dbfa398b2856f907 = function() { return handleError(function (arg0, arg1) {
        arg0.append(arg1);
    }, arguments) };
    imports.wbg.__wbg_arc_c46ca66b5ec2f1ac = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.arc(arg1, arg2, arg3, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_beginPath_08eae248f93ea32d = function(arg0) {
        arg0.beginPath();
    };
    imports.wbg.__wbg_body_544738f8b03aef13 = function(arg0) {
        const ret = arg0.body;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_call_abb4ff46ce38be40 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createElement_da4ed2b219560fc6 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_5b745e82ba551ca5 = function(arg0) {
        const ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_fillRect_84131220403e26a4 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_fillText_56566d8049e84e17 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_fill_dd0f756eea36e037 = function(arg0) {
        arg0.fill();
    };
    imports.wbg.__wbg_getContext_01f42b234e833f0a = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_getTimezoneOffset_45389e26d6f46823 = function(arg0) {
        const ret = arg0.getTimezoneOffset();
        return ret;
    };
    imports.wbg.__wbg_height_a07787f693c253d2 = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_d070139aaac1459f = function(arg0) {
        let result;
        try {
            result = arg0 instanceof CanvasRenderingContext2D;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlElement_20a3acb594113d73 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof HTMLElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_b5cf7783caa68180 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_lineTo_4b884d8cebfc8c54 = function(arg0, arg1, arg2) {
        arg0.lineTo(arg1, arg2);
    };
    imports.wbg.__wbg_moveTo_36127921f1ca46a5 = function(arg0, arg1, arg2) {
        arg0.moveTo(arg1, arg2);
    };
    imports.wbg.__wbg_new_0_23cedd11d9b40c9d = function() {
        const ret = new Date();
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_new_no_args_cb138f77cf6151ee = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_now_69d776cd24f5215b = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_offsetHeight_3a0cb1c43babb670 = function(arg0) {
        const ret = arg0.offsetHeight;
        return ret;
    };
    imports.wbg.__wbg_offsetWidth_058fe16c7dd9e9bb = function(arg0) {
        const ret = arg0.offsetWidth;
        return ret;
    };
    imports.wbg.__wbg_remove_32f69ffabcbc4072 = function(arg0) {
        arg0.remove();
    };
    imports.wbg.__wbg_restore_6486cb1a7aa3af7b = function(arg0) {
        arg0.restore();
    };
    imports.wbg.__wbg_rotate_4185d7f8614ba2d5 = function() { return handleError(function (arg0, arg1) {
        arg0.rotate(arg1);
    }, arguments) };
    imports.wbg.__wbg_save_b8767cfd2ee7f600 = function(arg0) {
        arg0.save();
    };
    imports.wbg.__wbg_setAttribute_34747dd193f45828 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_set_fillStyle_ea371e123273908b = function(arg0, arg1) {
        arg0.fillStyle = arg1;
    };
    imports.wbg.__wbg_set_font_37c5ab71d0189314 = function(arg0, arg1, arg2) {
        arg0.font = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_lineWidth_feda4b79a15c660b = function(arg0, arg1) {
        arg0.lineWidth = arg1;
    };
    imports.wbg.__wbg_set_strokeStyle_857faae3a756ddf4 = function(arg0, arg1) {
        arg0.strokeStyle = arg1;
    };
    imports.wbg.__wbg_set_textAlign_5d82eb01e9d2291e = function(arg0, arg1, arg2) {
        arg0.textAlign = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_textBaseline_9e8ed61033c5023d = function(arg0, arg1, arg2) {
        arg0.textBaseline = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_textContent_e461237efe237e01 = function(arg0, arg1, arg2) {
        arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_769e6b65d6557335 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_08f5a74c69739274 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_a8924b26aa92d024 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_stringify_655a6390e1f5eb6b = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_strokeRect_31a396bc4462b669 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.strokeRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_stroke_a18b81eb49ff370e = function(arg0) {
        arg0.stroke();
    };
    imports.wbg.__wbg_toDataURL_0e3da1a2eb93763f = function() { return handleError(function (arg0, arg1) {
        const ret = arg1.toDataURL();
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_translate_5c51221dc69f0baa = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.translate(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_width_dd0cfe94d42f5143 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('cstimer_analyzer_web_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
