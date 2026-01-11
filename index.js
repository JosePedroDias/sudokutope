let state = new Array(40).fill(true);
{
    for (let i = 0; i < 40; i++) state[i] = undefined;
}

let cellGroups = {};

////

const LS_KEY = 'SDKTP';

function load() {
    const s = localStorage.getItem(LS_KEY);
    if (s) {
        const o = JSON.parse(s);
        state = o.map(v => v === null ? undefined : v);
        for (let i = 0; i < 40; i++) setLabel(i, state[i] || '');
    }
}

function save() {
    const s = JSON.stringify(state);
    localStorage.setItem(LS_KEY, s);
}

////

let isMobile = false;
let svgScale = 1;
if (window.innerWidth <= 768) {
    isMobile = true;
    const w = window.innerWidth;
    const h = window.innerHeight;
    const svgW = 500;
    const s = Math.min(w, h);
    svgScale = s / svgW;
}

document.body.classList.add(isMobile ? 'mobile' : 'desktop');

////

function setColor(cell, r, g, b) {
    const f = cell.getAttribute('fill');
    const R = r !== undefined ? r : f.slice(1, 3);
    const G = g !== undefined ? g : f.slice(3, 5);
    const B = b !== undefined ? b : f.slice(5, 7);
    cell.setAttribute('fill', `#${R}${G}${B}`);
}

function inWhichPetal(cellId) {
    for (let i = 0; i < 5; i++) {
        if (cellGroups[`petal${i}`].includes(cellId)) return i;
    }
    return -1;
}

function cellFromId(id) {
    return document.getElementById(`cell${id}`);
}

function petalFromId(id) {
    return document.getElementById(`zone${id}`);
}

function setLabel(id, text) {
    const el = document.getElementById(`label${id}`);
    el.textContent = text;
}

function clear() {
    for (let i = 0; i < 40; i++) {
        cellFromId(i).setAttribute('fill', '#777777');
        cellFromId(i).setAttribute('stroke', '#000000');
    }
}

function highlightPetal(petalId) {
    for (let i = 0; i < 5; i++) {
        const highlight = i === petalId;
        petalFromId(i).setAttribute('stroke', highlight ? '#0000FF' : '#000000');
        petalFromId(i).setAttribute('stroke-width', highlight ? 6 : 2);
    }
}

function check(fromIdx) {
    clear();
    let incomplete = false;
    let ok = true;
    for (let [name, g] of Object.entries(cellGroups)) {
        const seenIndices = [];
        const seen = [];
        for (let c of g) {
            const v = state[c];
            if (v === undefined) {
                incomplete = true;
                continue;
            }
            const seenIdx = seen.indexOf(v);
            if (seenIdx!== -1) {
                //console.log('duplicate', v, 'in', name, g);
                cellFromId(c).setAttribute('fill', '#FFAAAA');
                cellFromId(seenIndices[seenIdx]).setAttribute('fill', '#FFAAAA');
                ok = false;
                break;
            }
            seen.push(v);
            seenIndices.push(c);
        }
    }
    if (ok) {
        if (!incomplete) {
            highlightPetal(-1);
            for (let i = 0; i < 40; i++) cellFromId(i).setAttribute('fill', '#00FF00');
        } else {
            colorize(fromIdx);
        }
    }
    return ok;
}

function colorize(cellNumber) {
    clear();
    let colors = [
        ['FF', undefined, undefined],
        [undefined, 'FF', undefined],
        [undefined, undefined, 'FF'],
    ];
    let cIdx = 0;

    const petalId = inWhichPetal(cellNumber);
    highlightPetal(petalId);
    for (const [name, cells] of Object.entries(cellGroups)) {
        if (name.startsWith('petal')) continue;
        if (cells.includes(cellNumber)) {
            //console.log('name', name);//, cells);
            let [r, g, b] = colors[cIdx++];
            for (let c of cells) {
                if (c !== cellNumber) {
                    setColor(cellFromId(c), r, g, b);
                }
            }
        }
    }

    cellFromId(cellNumber).setAttribute('fill', '#FFFFFF');
    selectedIndex = cellNumber;
    //console.log(`Cell ${cellNumber}`);
}

let selectedIndex = -1;

Promise.all([
    fetch('data40.json').then(response => response.json()),
    fetch('grid40.svg').then(response => response.text()),
])
    .then(([data, svgText]) => {
        cellGroups = data.constrants;

        // parse the SVG text into a DOM element and add it to the page
        const parser = new DOMParser();
        const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');
        const svgElement = svgDoc.documentElement;

        svgElement.setAttribute('viewBox', '0 0 500 487');

        if (isMobile && svgScale < 1) {
            svgElement.setAttribute('width', Math.round(500 * svgScale));
            svgElement.setAttribute('height', Math.round(487 * svgScale));
        }

        document.body.appendChild(svgElement);

        // make text elements not receive pointer events
        const textElements = svgElement.querySelectorAll('text');
        textElements.forEach(text => {
            text.style.pointerEvents = 'none';
        });

        clear();
        for (let i = 0; i < 40; i++) setLabel(i, state [i] || '');
        load();

        // add click event listener to the SVG
        svgElement.addEventListener('click', (ev) => {
            const target = ev.target;
            if (target.tagName === 'path' && target.id && target.id.startsWith('cell')) {
                const cellNumber = parseInt(target.id.replace('cell', ''));
                colorize(cellNumber);
            }
        });
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });


const keys = ['1', '2', '3', '4', '5', '6', '7', '8', 'Backspace'];

function onNumberInput(k) {
    let prev = state[selectedIndex];
    setLabel(selectedIndex, k);
    state[selectedIndex] = parseInt(k, 10);
    if (!check(selectedIndex)) state[selectedIndex] = prev;
    save();
}

function onClearCellInput() {
    setLabel(selectedIndex, '');
    state[selectedIndex] = undefined;
    colorize(selectedIndex);
    save();
}

document.addEventListener('keydown', (ev) => {
    if (selectedIndex === -1) return;
    const k = ev.key;
    if (ev.metaKey || ev.ctrlKey || ev.altKey) return;
    //console.log('key', k);
    if (keys.includes(k)) {
        if (k === 'Backspace') {
            onClearCellInput();
        } else {
            onNumberInput(k);
        }
        ev.preventDefault();
        ev.stopPropagation();
    }
});

if (isMobile) {
    const uiEl = document.querySelector('.ui');
    uiEl.addEventListener('click', (ev) => {
        const targetEl = ev.target;
        if (targetEl.tagName.toLowerCase() !== 'button') return;
        const isClear = targetEl.id === 'clear';
        if (isClear) {
            onClearCellInput();
        } else {
            onNumberInput(targetEl.textContent);
        }
    });
}
