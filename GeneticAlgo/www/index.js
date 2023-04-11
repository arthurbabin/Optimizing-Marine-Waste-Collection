import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();

document.getElementById('train').onclick = function() {
    for (let i = 0; i < 10; i++) {
        console.log(train());
    }
};

// Test Wasm binding
const sea = simulation.sea();
console.log(sea);

// Viewport config
const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportWidth * viewportScale;
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportWidth + 'px';

const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);
ctxt.fillStyle = 'rgb(0, 0, 0)';


CanvasRenderingContext2D.prototype.drawCollector=
    function (x, y, size, rotation) {
        this.lineWidth=3;
        this.beginPath();
        this.arc(
            x,
            y,
            size,
            rotation + Math.PI ,
            rotation,
        );
        this.strokeStyle = "#FFFFFF";
        this.stroke();
        this.fillStyle = '#fa6400';
        this.fill();

        this.beginPath();
        this.arc(
            x,
            y,
            size, 
            rotation-1,
            rotation+1,
        );
        this.stroke();
        this.fillStyle = '#fa6400';
        this.fill();

        this.beginPath();
        this.arc(
            x,
            y,
            size, 
            rotation + Math.PI - 1, 
            rotation + Math.PI +1, 
        );
        this.stroke();
        this.fillStyle = '#fa6400';
        this.fill();
    };

CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);

        this.fillStyle = '#f1c40f';
        this.fill();
    };

function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportWidth);

    for (let i = 0; i < 1; i++) {
        simulation.step();
    }

    const sea = simulation.sea();

    for (const waste of sea.wastes) {
        ctxt.drawCircle(
            waste.x * viewportWidth,
            waste.y * viewportWidth,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    for (const collector of sea.collectors) {
        ctxt.drawCollector(
            collector.x * viewportWidth,
            collector.y * viewportWidth,
            0.013 * viewportWidth,
            collector.rotation,
        );
    }

    // requestAnimationFrame() schedules code only for the next frame.
    //
    // Because we want for our simulation to continue forever, we've
    // gotta keep re-scheduling our function:
    requestAnimationFrame(redraw);
}

redraw();

const statChart = new Chart("stats", {
    type: "line",
    data: {
        labels: [0],
        datasets: [{
            label: 'Min',
            data: [0],
            fill: false,
            borderColor: '#fa6400',
            tension: 0.1,
        },{
            label: 'Max',
            data: [0],
            fill: false,
            borderColor: '#F7DB3C',
            tension: 0.1
        },{
            label: 'Avg',
            data: [0],
            fill: false,
            borderColor: '#654FF0',
            tension: 0.1
        }]
    },
    options: {
        responsive: true,
        aspectRatio: 1.3,
        title: {
            display: true,
            text: 'Collected wastes per unit',
            fontSize: 30,
        },
        scales: {
            yAxes: [{
                ticks: {
                    beginAtZero: true
                }
            }]
        }
    }
});

function addData(chart, traindata) {
    chart.data.labels.push(chart.data.labels.length);
    chart.data.datasets[0].data.push(traindata["min"]);
    chart.data.datasets[1].data.push(traindata["max"]);
    chart.data.datasets[2].data.push(traindata["avg"]);
    chart.update();
}

function removeData(chart) {
    chart.data.labels.pop();
    chart.data.datasets.forEach((dataset) => {
        dataset.data.pop();
    });
    chart.update();
}

function train() {
    let data = simulation.train();
    const regex = /(\w+)=(\d+\.\d+)/g;
    const matches = data.matchAll(regex);

    const result = {};
    for (const match of matches) {
        result[match[1]] = parseFloat(match[2]);
    }
    addData(statChart, result)
    return result;
}
