(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/mapgen_demo.js":
/*!*****************************!*\
  !*** ../pkg/mapgen_demo.js ***!
  \*****************************/
/*! exports provided: main, add, __wbindgen_object_drop_ref, __wbg_instanceof_Window_adf3196bdc02b386, __wbg_document_6cc8d0b87c0a99b9, __wbg_body_8c888fe47d81765f, __wbg_createElement_5bdf88a5af9f17c5, __wbg_setinnerHTML_4ff235db1a3cb4d8, __wbg_appendChild_77215fd672b162c5, __wbg_call_8e95613cc6524977, __wbindgen_object_clone_ref, __wbg_newnoargs_f3b8a801d5d4b079, __wbg_self_07b2f89e82ceb76d, __wbg_window_ba85d88572adc0dc, __wbg_globalThis_b9277fc37e201fe5, __wbg_global_e16303fe83e1d57f, __wbindgen_is_undefined, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./mapgen_demo_bg.wasm */ \"../pkg/mapgen_demo_bg.wasm\");\n/* harmony import */ var _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./mapgen_demo_bg.js */ \"../pkg/mapgen_demo_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"main\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"main\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"add\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"add\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_adf3196bdc02b386\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_instanceof_Window_adf3196bdc02b386\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_6cc8d0b87c0a99b9\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_document_6cc8d0b87c0a99b9\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_body_8c888fe47d81765f\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_body_8c888fe47d81765f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_createElement_5bdf88a5af9f17c5\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_createElement_5bdf88a5af9f17c5\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_setinnerHTML_4ff235db1a3cb4d8\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_setinnerHTML_4ff235db1a3cb4d8\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_appendChild_77215fd672b162c5\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_appendChild_77215fd672b162c5\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_8e95613cc6524977\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_call_8e95613cc6524977\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_clone_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_f3b8a801d5d4b079\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newnoargs_f3b8a801d5d4b079\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_07b2f89e82ceb76d\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_07b2f89e82ceb76d\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_ba85d88572adc0dc\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_window_ba85d88572adc0dc\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_b9277fc37e201fe5\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_globalThis_b9277fc37e201fe5\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_e16303fe83e1d57f\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_global_e16303fe83e1d57f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_rethrow\"]; });\n\n\n\n_mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]();\n\n\n//# sourceURL=webpack:///../pkg/mapgen_demo.js?");

/***/ }),

/***/ "../pkg/mapgen_demo_bg.js":
/*!********************************!*\
  !*** ../pkg/mapgen_demo_bg.js ***!
  \********************************/
/*! exports provided: main, add, __wbindgen_object_drop_ref, __wbg_instanceof_Window_adf3196bdc02b386, __wbg_document_6cc8d0b87c0a99b9, __wbg_body_8c888fe47d81765f, __wbg_createElement_5bdf88a5af9f17c5, __wbg_setinnerHTML_4ff235db1a3cb4d8, __wbg_appendChild_77215fd672b162c5, __wbg_call_8e95613cc6524977, __wbindgen_object_clone_ref, __wbg_newnoargs_f3b8a801d5d4b079, __wbg_self_07b2f89e82ceb76d, __wbg_window_ba85d88572adc0dc, __wbg_globalThis_b9277fc37e201fe5, __wbg_global_e16303fe83e1d57f, __wbindgen_is_undefined, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module, global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"main\", function() { return main; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"add\", function() { return add; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_adf3196bdc02b386\", function() { return __wbg_instanceof_Window_adf3196bdc02b386; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_6cc8d0b87c0a99b9\", function() { return __wbg_document_6cc8d0b87c0a99b9; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_body_8c888fe47d81765f\", function() { return __wbg_body_8c888fe47d81765f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_createElement_5bdf88a5af9f17c5\", function() { return __wbg_createElement_5bdf88a5af9f17c5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_setinnerHTML_4ff235db1a3cb4d8\", function() { return __wbg_setinnerHTML_4ff235db1a3cb4d8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_appendChild_77215fd672b162c5\", function() { return __wbg_appendChild_77215fd672b162c5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_8e95613cc6524977\", function() { return __wbg_call_8e95613cc6524977; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_f3b8a801d5d4b079\", function() { return __wbg_newnoargs_f3b8a801d5d4b079; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_07b2f89e82ceb76d\", function() { return __wbg_self_07b2f89e82ceb76d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_ba85d88572adc0dc\", function() { return __wbg_window_ba85d88572adc0dc; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_b9277fc37e201fe5\", function() { return __wbg_globalThis_b9277fc37e201fe5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_e16303fe83e1d57f\", function() { return __wbg_global_e16303fe83e1d57f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return __wbindgen_rethrow; });\n/* harmony import */ var _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./mapgen_demo_bg.wasm */ \"../pkg/mapgen_demo_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction main() {\n    _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"main\"]();\n}\n\n/**\n* @param {number} a\n* @param {number} b\n* @returns {number}\n*/\nfunction add(a, b) {\n    var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"add\"](a, b);\n    return ret >>> 0;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_instanceof_Window_adf3196bdc02b386 = function(arg0) {\n    var ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nconst __wbg_document_6cc8d0b87c0a99b9 = function(arg0) {\n    var ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __wbg_body_8c888fe47d81765f = function(arg0) {\n    var ret = getObject(arg0).body;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nconst __wbg_createElement_5bdf88a5af9f17c5 = handleError(function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n});\n\nconst __wbg_setinnerHTML_4ff235db1a3cb4d8 = function(arg0, arg1, arg2) {\n    getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);\n};\n\nconst __wbg_appendChild_77215fd672b162c5 = handleError(function(arg0, arg1) {\n    var ret = getObject(arg0).appendChild(getObject(arg1));\n    return addHeapObject(ret);\n});\n\nconst __wbg_call_8e95613cc6524977 = handleError(function(arg0, arg1) {\n    var ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n});\n\nconst __wbindgen_object_clone_ref = function(arg0) {\n    var ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_newnoargs_f3b8a801d5d4b079 = function(arg0, arg1) {\n    var ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_self_07b2f89e82ceb76d = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_window_ba85d88572adc0dc = handleError(function() {\n    var ret = window.window;\n    return addHeapObject(ret);\n});\n\nconst __wbg_globalThis_b9277fc37e201fe5 = handleError(function() {\n    var ret = globalThis.globalThis;\n    return addHeapObject(ret);\n});\n\nconst __wbg_global_e16303fe83e1d57f = handleError(function() {\n    var ret = global.global;\n    return addHeapObject(ret);\n});\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbindgen_rethrow = function(arg0) {\n    throw takeObject(arg0);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module), __webpack_require__(/*! ./../www/node_modules/webpack/buildin/global.js */ \"./node_modules/webpack/buildin/global.js\")))\n\n//# sourceURL=webpack:///../pkg/mapgen_demo_bg.js?");

/***/ }),

/***/ "../pkg/mapgen_demo_bg.wasm":
/*!**********************************!*\
  !*** ../pkg/mapgen_demo_bg.wasm ***!
  \**********************************/
/*! exports provided: memory, main, add, __wbindgen_exn_store, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./mapgen_demo_bg.js */ \"../pkg/mapgen_demo_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/mapgen_demo_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var mapgen_demo__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! mapgen-demo */ \"../pkg/mapgen_demo.js\");\n\n\nconst x = mapgen_demo__WEBPACK_IMPORTED_MODULE_0__[\"add\"](1, 4);\nconsole.log(x);\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/global.js":
/*!***********************************!*\
  !*** (webpack)/buildin/global.js ***!
  \***********************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("var g;\n\n// This works in non-strict mode\ng = (function() {\n\treturn this;\n})();\n\ntry {\n\t// This works if eval is allowed (see CSP)\n\tg = g || new Function(\"return this\")();\n} catch (e) {\n\t// This works if the window reference is available\n\tif (typeof window === \"object\") g = window;\n}\n\n// g can still be undefined, but nothing to do about it...\n// We return undefined, instead of nothing here, so it's\n// easier to handle this case. if(!global) { ...}\n\nmodule.exports = g;\n\n\n//# sourceURL=webpack:///(webpack)/buildin/global.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);