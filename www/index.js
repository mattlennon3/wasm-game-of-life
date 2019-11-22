import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const cell_size = 5;
const grid_colour =  '#CCCCCC';
const dead_colour =  '#FFFFFF';
const alive_colour = '#000000';

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('game-of-life-canvas');
canvas.height = ((cell_size + 1) * height) + 1;
canvas.width = ((cell_size + 1) * width) + 1;

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = grid_colour;

    // Vertical Lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (cell_size + 1) + 1, 0);
        ctx.lineTo(i * (cell_size + 1), (cell_size + 1) * height + 1 );
    }

    // Horizontal Lines
    for (let j = 0; j <= height; j++ ) {
        ctx.moveTo(0, j * (cell_size + 1) + 1);
        ctx.lineTo((cell_size + 1) * width + 1, j * (cell_size + 1) + 1 );
    }

    ctx.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for(let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            ctx.fillStyle = (cells[index] === Cell.Dead) ? dead_colour : alive_colour;

            ctx.fillRect(
                col * (cell_size + 1) + 1,
                row * (cell_size + 1) + 1,
                cell_size,
                cell_size
            )
        }
    }

    ctx.stroke();
}




drawGrid();
drawCells();
requestAnimationFrame(renderLoop);












