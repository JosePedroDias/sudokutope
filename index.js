let mode = 60;
if (location.hash === '#40') mode = 40;
let numPetals = mode === 40 ? 5 : 6;

let selectedIndex = -1;
let state = new Array(mode).fill(true);
{
    for (let i = 0; i < mode; i++) state[i] = undefined;
}
let cellGroups = {};

////

const LS_KEY = `SDKTP${mode}`;

function load() {
    const s = localStorage.getItem(LS_KEY);
    if (s) {
        const o = JSON.parse(s);
        state = o.map(v => v === null ? undefined : v);
        for (let i = 0; i < mode; i++) setLabel(i, state[i] || '');
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
document.body.classList.add(`mode${mode}`);

////

function setColor(cell, r, g, b) {
    const f = cell.getAttribute('fill');
    const R = r !== undefined ? r : f.slice(1, 3);
    const G = g !== undefined ? g : f.slice(3, 5);
    const B = b !== undefined ? b : f.slice(5, 7);
    cell.setAttribute('fill', `#${R}${G}${B}`);
}

function inWhichPetal(cellId) {
    for (let i = 0; i < numPetals; i++) {
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
    for (let i = 0; i < mode; i++) {
        const cell = cellFromId(i);
        cell.setAttribute('fill', '#777777');
        cell.setAttribute('stroke', '#000000');
    }
}

function highlightPetal(petalId) {
    for (let i = 0; i < numPetals; i++) {
        const highlight = i === petalId;
        const petal = petalFromId(i);
        petal.setAttribute('stroke', highlight ? '#0000FF' : '#000000');
        petal.setAttribute('stroke-width', highlight ? 6 : 2);
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
            for (let i = 0; i < mode; i++) cellFromId(i).setAttribute('fill', '#00FF00');
        } else {
            colorize(fromIdx);
        }
    }
    return ok;
}

let colors = [
    ['FF', undefined, undefined],
    [undefined, 'FF', undefined],
    [undefined, undefined, 'FF'], // unused ATM
];
function colorize(cellNumber) {
    clear();
    
    let cIdx = 0;
    const petalId = inWhichPetal(cellNumber);
    highlightPetal(petalId);
    for (const [name, cells] of Object.entries(cellGroups)) {
        if (cells.includes(cellNumber)) {
            //console.log(`- ${name}`);
            if (name.startsWith('petal')) continue;
            //console.log(`- ${}', name);//, cells);
            let [r, g, b] = colors[cIdx++];
            for (let c of cells) {
                if (c !== cellNumber) setColor(cellFromId(c), r, g, b);
            }
        }
    }

    cellFromId(cellNumber).setAttribute('fill', '#FFFFFF');
}

Promise.all([
    fetch(`data${mode}.json`).then(response => response.json()),
    fetch(`grid${mode}.svg`).then(response => response.text()),
])
    .then(([data, svgText]) => {
        cellGroups = data.constraints;

        const parser = new DOMParser();
        const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');
        const svgElement = svgDoc.documentElement;

        const h = mode === 40 ? 487 : 492;
        svgElement.setAttribute('viewBox', `0 0 500 ${h}`);

        if (isMobile && svgScale < 1) {
            svgElement.setAttribute('width', Math.round(500 * svgScale));
            svgElement.setAttribute('height', Math.round(h * svgScale));
        }

        document.body.appendChild(svgElement);

        const textElements = svgElement.querySelectorAll('text');
        textElements.forEach(text => {
            text.style.pointerEvents = 'none';
        });
        //return;
        clear();
        for (let i = 0; i < mode; i++) setLabel(i, state [i] || '');
        load();

        svgElement.addEventListener('click', (ev) => {
            const target = ev.target;
            if (target.tagName === 'path' && target.id && target.id.startsWith('cell')) {
                const cellNumber = parseInt(target.id.replace('cell', ''));
                selectedIndex = cellNumber;
                colorize(cellNumber);
            }
        });
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });


const keys = (mode === 40) ? ['1', '2', '3', '4', '5', '6', '7', '8', 'Backspace', 'Escape'] : ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'Backspace', 'Escape'];

function onNumberInput(k) {
    if (selectedIndex === -1) return;
    let prev = state[selectedIndex];
    setLabel(selectedIndex, k);
    state[selectedIndex] = parseInt(k, 10);
    if (!check(selectedIndex)) state[selectedIndex] = prev;
    save();
}

function onClearCellInput() {
    if (selectedIndex === -1) return;
    setLabel(selectedIndex, '');
    state[selectedIndex] = undefined;
    colorize(selectedIndex);
    save();
}

function onNewInput() {
    for (let i = 0; i < mode; i++) {
        state[i] = undefined;
        setLabel(i, '');
    }
    selectedIndex = -1;
    clear();
    save();
}

document.addEventListener('keydown', (ev) => {
    if (selectedIndex === -1) return;
    const k = ev.key;
    if (ev.metaKey || ev.ctrlKey || ev.altKey) return;
    //console.log('key', k);
    if (keys.includes(k)) {
        if (k === 'Backspace') onClearCellInput();
        else if (k === 'Escape') onNewInput();
        else onNumberInput(k);
        ev.preventDefault();
        ev.stopPropagation();
    }
});

if (isMobile) {
    const uiEl = document.querySelector('.ui');

    if (mode === 40) {
        const uiButtonEls = Array.from(document.querySelectorAll('.ui button'));
        for (let el of uiButtonEls) {
            if (el.textContent === '0' || el.textContent === '9') el.parentNode.removeChild(el);
        }
    }
    
    uiEl.addEventListener('click', (ev) => {
        const targetEl = ev.target;
        if (targetEl.tagName.toLowerCase() !== 'button') return;
        const isClear = targetEl.id === 'clear';
        const isNew = targetEl.id === 'new';
        if (isClear) onClearCellInput();
        else if (isNew) onNewInput();
        else onNumberInput(targetEl.textContent);
    });
}
