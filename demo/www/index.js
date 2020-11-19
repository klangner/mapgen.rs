import {Cell, World} from "mapgen-demo";
import { memory } from "mapgen-demo/mapgen_demo_bg";

const CANVAS_SIZE = 750;
const GRID_COLS = 80;
const GRID_ROWS = 50;
const CELL_SIZE = CANVAS_SIZE/GRID_ROWS;
const TILE_SIZE = 39;

// Init canvas
const canvas = document.getElementById("mapgen-canvas");
canvas.height = CELL_SIZE * GRID_ROWS;
canvas.width = CELL_SIZE * GRID_COLS;
const ctx = canvas.getContext('2d');
// Info box
const infoDiv = document.getElementById('map-info');
// API to the WASM
let world = null;

// Load tiles bitmap
let tiles_image = new Image();
tiles_image.src = 'assets/tiles.png';

// Take provided seed or generate new one
function get_seed() {
    var seed_text = document.getElementById("seed").value;
    if( seed_text.length > 0) {
        return Number(seed_text);
    } 
    return Date.now();
}

// Map generators
function newCellularAutomata() {
    world = World.new_cellular_automata(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newSimpleRooms() {
    var seed = Date.now();
    world = World.new_simple_rooms(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newBspInterior() {
    var seed = Date.now();
    world = World.new_bsp_interior(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newDrunkard() {
    var seed = Date.now();
    world = World.new_drunkard(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newBspRooms() {
    var seed = Date.now();
    world = World.new_bsp_rooms(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newMaze() {
    var seed = Date.now();
    world = World.new_maze(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newVoronoi() {
    var seed = Date.now();
    world = World.new_voronoi(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

function newRandomGen() {
    var seed = Date.now();
    world = World.new_random(GRID_COLS, GRID_ROWS, get_seed());
    requestAnimationFrame(renderLoop);
}

const renderLoop = () => {
    // universe.tick();
    drawCells();
    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * GRID_COLS + column;
};

const is_inner_wall = (tiles, col, row) => {
    for (let c = Math.max(col - 1, 0); c < Math.min(col + 2, GRID_COLS); c++) {
        for (let r = Math.max(row - 1, 0); r < Math.min(row + 2, GRID_ROWS); r++) {
            if ((c != col || r != row) && tiles[getIndex(r, c)] == Cell.Floor) {
                return false;
            }
        }
    }

    return true;
}

const draw_tile = (ctx, row, col, tile_type) => {
    var tile_x = 0;
    var tile_y = 0;
    if (tile_type == "floor") {
        tile_x = 3;
        tile_y = 2;
    } else if (tile_type == "wall") {
        tile_x = 0;
        tile_y = 3;
    } else if (tile_type == "player") {
        tile_x = 0;
        tile_y = 8;
    } else if (tile_type == "exit") {
        tile_x = 10;
        tile_y = 1;
    } else {
        tile_x = 18;
        tile_y = 0;
    }

    ctx.drawImage(
        tiles_image,
        tile_x * TILE_SIZE + 3,
        tile_y * TILE_SIZE + 3,
        TILE_SIZE - 3,
        TILE_SIZE - 3,
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE);

}

const drawCells = () => {
    const tilesPtr = world.tiles();
    const tiles = new Uint8Array(memory.buffer, tilesPtr, GRID_COLS * GRID_ROWS);

    // tiles
    for (let row = 0; row < GRID_ROWS; row++) {
        for (let col = 0; col < GRID_COLS; col++) {
            const idx = getIndex(row, col);
            if (tiles[idx] == Cell.Floor) {
                draw_tile(ctx, row, col, "floor");
            } else if (is_inner_wall(tiles, col, row)){
                draw_tile(ctx, row, col, "inner-wall");
            } else {
                draw_tile(ctx, row, col, "wall");
            }
        }
    }

    // Player position
    let player = world.player_pos();
    draw_tile(ctx, player.row(), player.col(), "player");

    // Exit position
    let exit = world.exit_pos();
    draw_tile(ctx, exit.row(), exit.col(), "exit");
};

newRandomGen();

// Connect UI element
document.getElementById('cellular-automata-option').addEventListener('click', newCellularAutomata);
document.getElementById('simple-rooms-option').addEventListener('click', newSimpleRooms);
document.getElementById('bsp-rooms-option').addEventListener('click', newBspRooms);
document.getElementById('drunkard-option').addEventListener('click', newDrunkard);
document.getElementById('bsp-interior-option').addEventListener('click', newBspInterior);
document.getElementById('maze-option').addEventListener('click', newMaze);
document.getElementById('voronoi-option').addEventListener('click', newVoronoi);
document.getElementById('random-option').addEventListener('click', newRandomGen);
