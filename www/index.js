import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const testdiv = document.getElementById("test");
const universe = Universe.new();

// for(var i = 0; i < 100; i++) {
//     var line = document.createElement("p");
//     line.textContent = universe.render();
//     testdiv.appendChild(line);
//     universe.tick();
// }

const renderLoop = () => {
    pre.innerHTML = universe.render();
    universe.tick();
    // sleep(1)

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
