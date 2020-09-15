import {Cell, World} from "mapgen-demo";
import { memory } from "mapgen-demo/mapgen_demo_bg";

const CELL_SIZE = 12;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

var world = null;
const width = 80;
const height = 50;

const infoDiv = document.getElementById('map-info');
// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("mapgen-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

// Map generators
function newCellularAutomata() {
    var seed = Date.now();
    world = World.new_cellular_automata(width, height, seed);
    requestAnimationFrame(renderLoop);
}

function newSimpleRooms() {
    var seed = Date.now();
    world = World.new_simple_rooms(width, height, seed);
    requestAnimationFrame(renderLoop);
}

function newRandomGen() {
    var seed = Date.now();
    world = World.new_random(width, height, seed);
    requestAnimationFrame(renderLoop);
}

const renderLoop = () => {
    // universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const tilesPtr = world.tiles();
    const tiles = new Uint8Array(memory.buffer, tilesPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = tiles[idx] == Cell.Floor
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

newRandomGen();

// Connect UI element
document.getElementById('cellular-automata-option').addEventListener('click', newCellularAutomata);
document.getElementById('simple-rooms-option').addEventListener('click', newSimpleRooms);
document.getElementById('random-option').addEventListener('click', newRandomGen);
