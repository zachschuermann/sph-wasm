import { State } from "sph-wasm";
import { memory } from "sph-wasm/sph_wasm_bg";


const state = State.new();
// const width = state.width();
// const height = state.height();
const radius = 7;
const numParticles = 1000;

const canvas = document.getElementById("sph-canvas");

canvas.height = 600;
canvas.width = 800;

const ctx = canvas.getContext('2d');

ctx.transform(1, 0, 0, -1, 0, canvas.height)

const drawParticle = (x, y) => {
    ctx.beginPath();
    ctx.fillStyle = "#87edff";
    ctx.arc(x, y, radius, 0, Math.PI*2, true); 
    ctx.closePath();
    ctx.fill();
}

const drawParticles = () => {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const xPtr = state.xs();
    const xs = new Float32Array(memory.buffer, xPtr, numParticles * 2);

    for (let p = 0; p < numParticles; p++) {
        let x = xs[p];
        let y = xs[p + numParticles];
        drawParticle(x, y);
    }
};

const renderLoop = () => {
    state.update();
    drawParticles();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
