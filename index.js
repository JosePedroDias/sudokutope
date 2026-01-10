// Cell group definitions
// Each cell belongs to 1 petal (zone) and 2 arms
const cellGroups = {
    // Petal 0 - bottom right petal
    petal0: [1, 32, 35, 14, 13, 33, 34, 6],
    // Petal 1 - bottom left petal
    petal1: [0, 26, 25, 4, 12, 24, 18, 5],
    // Petal 2 - left petal
    petal2: [28, 20, 9, 29, 2, 8, 19, 27],
    // Petal 3 - top petal
    petal3: [22, 10, 21, 23, 11, 31, 30, 39],
    // Petal 4 - right petal
    petal4: [3, 37, 38, 17, 16, 15, 7, 36],

    // 10 arms
    arm0: [32, 33, 34, 35, 36, 37, 38, 39],
    arm1: [24, 12, 13, 14, 35, 15, 16, 17],
    arm2: [18, 25, 4, 5, 14, 34, 6, 7],
    arm3: [8, 19, 26, 0, 5, 13, 33, 1],
    arm4: [2, 9, 20, 27, 0, 4, 12, 32],
    arm5: [31, 30, 29, 28, 27, 26, 25, 24],
    arm6: [39, 23, 22, 21, 28, 20, 19, 18],
    arm7: [17, 38, 11, 10, 21, 29, 9, 8],
    arm8: [7, 16, 37, 3, 10, 22, 30, 2],
    arm8: [1, 6, 15, 36, 3, 11, 23, 31],
    arm9: [2, 30, 22, 10, 3, 37, 16, 7],
};

const state = new Array(40).fill(true);
{
    for (let i = 0; i < 40; i++) state[i] = undefined;
}


function setColor(cell, r, g, b) {
    const f = cell.getAttribute('fill');
    const R = r !== undefined ? r : f.slice(1, 3);
    const G = g !== undefined ? g : f.slice(3, 5);
    const B = b !== undefined ? b : f.slice(5, 7);
    cell.setAttribute('fill', `#${R}${G}${B}`);
}

function cellFromId(id) {
    return document.getElementById(`cell${id}`);
}

function setLabel(id, text) {
    const el = document.getElementById(`label${id}`);
    el.textContent = text;
}

function clear() {
    for (let i = 0; i < 40; i++) cellFromId(i).setAttribute('fill', '#555555');
}

let selectedIndex = -1;

fetch('grid5.svg')
    .then(response => response.text())
    .then(svgText => {
        // parse the SVG text into a DOM element and add it to the page
        const parser = new DOMParser();
        const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');
        const svgElement = svgDoc.documentElement;
        document.body.appendChild(svgElement);

        // make text elements not receive pointer events
        const textElements = svgElement.querySelectorAll('text');
        textElements.forEach(text => {
            text.style.pointerEvents = 'none';
        });

        clear();
        for (let i = 0; i < 40; i++) setLabel(i, state [i] || '');

        // add click event listener to the SVG
        svgElement.addEventListener('click', (ev) => {
            const target = ev.target;

            if (target.tagName === 'path' && target.id && target.id.startsWith('cell')) {
                const cellNumber = parseInt(target.id.replace('cell', ''));

                clear();

                let colors = [
                    ['FF', undefined, undefined],
                    [undefined, 'FF', undefined],
                    [undefined, undefined, 'FF'],
                ];
                let cIdx = 0;

                for (const [groupName, cells] of Object.entries(cellGroups)) {
                    if (cells.includes(cellNumber)) {
                        //console.log('groupName', groupName);//, cells);
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
        });
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });


const keys = ['1', '2', '3', '4', '5', '6', '7', '8', 'Backspace'];

document.addEventListener('keydown', (ev) => {
    if (selectedIndex === -1) return;
    const k = ev.key;
    if (ev.metaKey || ev.ctrlKey || ev.altKey) return;
    if (keys.includes(k)) {
        //console.log('key', k);
        if (k === 'Backspace') {
            setLabel(selectedIndex, '');
        } else {
            setLabel(selectedIndex, k);
        }
        ev.preventDefault();
        ev.stopPropagation();
    }
});
