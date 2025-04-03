


const canvasWidth = 2000;
const canvasHeight = 2000;
const pointSize = 20;

const canvas = document.getElementById("area");
const ctx = canvas.getContext("2d");

let selection = -1;

let mouse = { x: 0, y: 0 };

let points = [
    { x: 200, y: 200 },
    { x: 1840, y: 240 },
    { x: 1200, y: 640 },
    { x: 1800, y: 800 },
];

draw();

canvas.addEventListener("mousedown", (e) => {
    for (let i = 0; i < points.length; i++) {
        const dx = mouse.x - points[i].x;
        const dy = mouse.y - points[i].y;
        if (Math.pow(dx, 2) + Math.pow(dy, 2) < Math.pow(pointSize, 2)) {
            selection = i;
        }
    }
})

window.addEventListener("mouseup", (e) => {
    selection = -1;
}, false)

canvas.addEventListener("mousemove", (e) => {
    console.log(selection);
    mouse.x = e.offsetX / canvas.clientWidth * canvasWidth;
    mouse.y = e.offsetY / canvas.clientHeight * canvasHeight;

    if (selection != -1) {
        points[selection].x = mouse.x;
        points[selection].y = mouse.y;
    }

    draw();
})


function draw() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const start = points[0];
    const cp1 = points[1];
    const cp2 = points[2];
    const end = points[3];

    // curve
    ctx.strokeStyle = "white";
    ctx.lineWidth = 10;
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y);
    ctx.stroke();

    // archors
    ctx.fillStyle = "#5555ff";
    ctx.beginPath();
    ctx.arc(start.x, start.y, pointSize, 0, 2 * Math.PI);
    ctx.arc(end.x, end.y, pointSize, 0, 2 * Math.PI);
    ctx.fill();

    // control
    ctx.fillStyle = "#ff5555";
    ctx.beginPath();
    ctx.arc(cp1.x, cp1.y, pointSize, 0, 2 * Math.PI);
    ctx.arc(cp2.x, cp2.y, pointSize, 0, 2 * Math.PI);
    ctx.fill();
}

