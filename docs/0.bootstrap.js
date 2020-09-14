(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/mapgen_demo.js":
/*!*****************************!*\
  !*** ../pkg/mapgen_demo.js ***!
  \*****************************/
/*! exports provided: Cell, World, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./mapgen_demo_bg.wasm */ \"../pkg/mapgen_demo_bg.wasm\");\n/* harmony import */ var _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./mapgen_demo_bg.js */ \"../pkg/mapgen_demo_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Cell\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Cell\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"World\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"World\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _mapgen_demo_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/mapgen_demo.js?");

/***/ }),

/***/ "../pkg/mapgen_demo_bg.js":
/*!********************************!*\
  !*** ../pkg/mapgen_demo_bg.js ***!
  \********************************/
/*! exports provided: Cell, World, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Cell\", function() { return Cell; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"World\", function() { return World; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./mapgen_demo_bg.wasm */ \"../pkg/mapgen_demo_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nconst Cell = Object.freeze({ Floor:0,\"0\":\"Floor\",Wall:1,\"1\":\"Wall\", });\n/**\n*/\nclass World {\n\n    static __wrap(ptr) {\n        const obj = Object.create(World.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_world_free\"](ptr);\n    }\n    /**\n    * @param {number} width\n    * @param {number} height\n    * @param {number} seed\n    * @returns {World}\n    */\n    static new_cellular_automata(width, height, seed) {\n        var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"world_new_cellular_automata\"](width, height, seed);\n        return World.__wrap(ret);\n    }\n    /**\n    * @param {number} width\n    * @param {number} height\n    * @param {number} seed\n    * @returns {World}\n    */\n    static new_random_rooms(width, height, seed) {\n        var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"world_new_random_rooms\"](width, height, seed);\n        return World.__wrap(ret);\n    }\n    /**\n    * @returns {number}\n    */\n    width() {\n        var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"world_width\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    height() {\n        var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"world_height\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    tiles() {\n        var ret = _mapgen_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"world_tiles\"](this.ptr);\n        return ret;\n    }\n}\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/mapgen_demo_bg.js?");

/***/ }),

/***/ "../pkg/mapgen_demo_bg.wasm":
/*!**********************************!*\
  !*** ../pkg/mapgen_demo_bg.wasm ***!
  \**********************************/
/*! exports provided: memory, __wbg_world_free, world_new_cellular_automata, world_new_random_rooms, world_width, world_height, world_tiles */
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
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var mapgen_demo__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! mapgen-demo */ \"../pkg/mapgen_demo.js\");\n/* harmony import */ var mapgen_demo_mapgen_demo_bg__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! mapgen-demo/mapgen_demo_bg */ \"../pkg/mapgen_demo_bg.wasm\");\n\n\n\nconst CELL_SIZE = 12;\nconst GRID_COLOR = \"#CCCCCC\";\nconst DEAD_COLOR = \"#FFFFFF\";\nconst ALIVE_COLOR = \"#000000\";\n\nvar world = null;\nconst width = 80;\nconst height = 50;\n\nconst infoDiv = document.getElementById('map-info');\n// Give the canvas room for all of our cells and a 1px border\n// around each of them.\nconst canvas = document.getElementById(\"mapgen-canvas\");\ncanvas.height = (CELL_SIZE + 1) * height + 1;\ncanvas.width = (CELL_SIZE + 1) * width + 1;\n\nconst ctx = canvas.getContext('2d');\n\n// Map generators\nfunction newCellularAutomata() {\n    var seed = Date.now();\n    world = mapgen_demo__WEBPACK_IMPORTED_MODULE_0__[\"World\"].new_cellular_automata(width, height, seed);\n    requestAnimationFrame(renderLoop);\n    infoDiv.textContent = \"Cellular Automata with the seed: \" + seed;\n}\n\nfunction newSimpleRooms() {\n    var seed = Date.now();\n    world = mapgen_demo__WEBPACK_IMPORTED_MODULE_0__[\"World\"].new_random_rooms(width, height, seed);\n    requestAnimationFrame(renderLoop);\n    infoDiv.textContent = \"Random Rooms with the seed: \" + seed;\n}\n\nconst renderLoop = () => {\n    // universe.tick();\n\n    drawGrid();\n    drawCells();\n\n    requestAnimationFrame(renderLoop);\n};\n\nconst drawGrid = () => {\n    ctx.beginPath();\n    ctx.strokeStyle = GRID_COLOR;\n\n    // Vertical lines.\n    for (let i = 0; i <= width; i++) {\n        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);\n        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);\n    }\n\n    // Horizontal lines.\n    for (let j = 0; j <= height; j++) {\n        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);\n        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);\n    }\n\n    ctx.stroke();\n};\n\nconst getIndex = (row, column) => {\n    return row * width + column;\n};\n\nconst drawCells = () => {\n    const tilesPtr = world.tiles();\n    const tiles = new Uint8Array(mapgen_demo_mapgen_demo_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, tilesPtr, width * height);\n\n    ctx.beginPath();\n\n    for (let row = 0; row < height; row++) {\n        for (let col = 0; col < width; col++) {\n            const idx = getIndex(row, col);\n\n            ctx.fillStyle = tiles[idx] == mapgen_demo__WEBPACK_IMPORTED_MODULE_0__[\"Cell\"].Floor\n                ? DEAD_COLOR\n                : ALIVE_COLOR;\n\n            ctx.fillRect(\n                col * (CELL_SIZE + 1) + 1,\n                row * (CELL_SIZE + 1) + 1,\n                CELL_SIZE,\n                CELL_SIZE\n            );\n        }\n    }\n\n    ctx.stroke();\n};\n\nnewCellularAutomata();\n\n// Connect UI element\ndocument.getElementById('cellular-automata-option').addEventListener('click', newCellularAutomata);\ndocument.getElementById('simple-rooms-option').addEventListener('click', newSimpleRooms);\n\n\n//# sourceURL=webpack:///./index.js?");

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