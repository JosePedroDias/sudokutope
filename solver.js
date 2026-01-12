const allConstraints = {
    40: [
        [ 1, 32, 35, 14, 13, 33, 34,  6],
        [ 0, 26, 25,  4, 12, 24, 18,  5],
        [28, 20,  9, 29,  2,  8, 19, 27],
        [22, 10, 21, 23, 11, 31, 30, 39],
        [ 3, 37, 38, 17, 16, 15,  7, 36],
        [32, 33, 34, 35, 36, 37, 38, 39],
        [24, 12, 13, 14, 35, 15, 16, 17],
        [18, 25,  4,  5, 14, 34,  6,  7],
        [ 8, 19, 26,  0,  5, 13, 33,  1],
        [ 2,  9, 20, 27,  0,  4, 12, 32],
        [31, 30, 29, 28, 27, 26, 25, 24],
        [39, 23, 22, 21, 28, 20, 19, 18],
        [17, 38, 11, 10, 21, 29,  9,  8],
        [ 1,  6, 15, 36,  3, 11, 23, 31],
        [ 2, 30, 22, 10,  3, 37, 16,  7]
    ],
    60: [
        [ 7, 15,  6,  1, 54, 53, 52, 51, 27, 26],
        [50, 25, 24, 40, 14, 13, 12, 41,  5,  4],
        [ 0, 44, 43, 42, 35, 34, 33, 32, 19, 18],
        [45, 20, 46, 36,  8,  9, 47, 37,  2, 48],
        [21, 10, 22, 11, 38, 23, 58, 49, 39, 59],
        [31, 57, 30, 17,  3, 56, 29, 16, 55, 28],
        [ 8,  9, 47, 37, 21, 10, 11, 57, 30, 17],
        [ 8, 19, 34, 43,  0,  5, 13, 25, 51,  1],
        [ 2, 48, 38, 22, 10,  3, 56, 29, 16,  7],
        [ 2,  9, 20, 35, 44,  0,  4, 12, 24, 50],
        [49, 39, 23, 11,  3, 55, 28, 15,  6,  1],
        [49, 48, 47, 46, 45, 44, 43, 42, 41, 40],
        [59, 58, 57, 56, 55, 54, 53, 52, 51, 50],
        [59, 39, 38, 37, 36, 45, 35, 34, 33, 32],
        [31, 30, 29, 28, 54, 27, 26, 25, 24, 40],
        [31, 58, 23, 22, 21, 36, 46, 20, 19, 18],
        [17, 16, 15, 53, 27, 14, 13, 12, 41, 32],
        [ 7,  6, 52, 26, 14,  5,  4, 42, 33, 18]
    ]
};

function randomizeArray(arrSet) {
    const arr = Array.from(arrSet);
    for (let i = arr.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [arr[i], arr[j]] = [arr[j], arr[i]];
    }
    return arr;
}

function oneOf(arr) {
    return arr[Math.floor(Math.random() * arr.length)];
}

function findIndices(arr, cb, ctx) {
    const res = [];
    for (let i = 0; i < arr.length; i++) {
        if (cb.call(ctx, arr[i], i, arr)) res.push(i);
    }
    return res;
}

function get8() {
    return new Set([1, 2, 3, 4, 5, 6, 7, 8]);
}

function get10() {
    return new Set([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

function isSolved(st) {
    return st.every((s) => typeof s === 'number');
}

function isValid(st, constraints) {
    for (const constraint of constraints) {
        const seen = new Set();
        for (const idx of constraint) {
            const v = st[idx];
            if (typeof v === 'number') {
                if (seen.has(v)) return false;
                seen.add(v);
                continue;
            }
        }
    }
    return true;
}

function trampoline(fn) {
    while (typeof fn === 'function') fn = fn();
    return fn;
}

function solveStep(maxTargetSize, constraints, st, lastChoices, stepNo) {
    console.log(`\n ** step ${stepNo} **`);

    if (isSolved(st)) {
        console.log('solved!');
        return st;
    }
    let targetSize = 2;
    let idx;
    let indices = [];

    // First try: avoid lastChoices
    while (targetSize <= maxTargetSize && indices.length === 0) {
        indices = findIndices(st, (s, i) => !lastChoices.includes(i) && typeof s !== 'number' && s.size === targetSize);
        if (indices.length === 0) {
            console.log(`no cell with size ${targetSize} (avoiding lastChoices)`);
            ++targetSize;
        }
    }

    // Second try: if nothing found, ignore lastChoices constraint
    if (indices.length === 0) {
        console.log('no cells found avoiding lastChoices, trying without that constraint');
        targetSize = 2;
        while (targetSize <= maxTargetSize && indices.length === 0) {
            indices = findIndices(st, (s, i) => typeof s !== 'number' && s.size === targetSize);
            if (indices.length === 0) {
                console.log(`no cell with size ${targetSize}`);
                ++targetSize;
            }
        }
    }

    if (indices.length > 0) {
        idx = oneOf(indices);
        console.log(`found ${indices.length} cells with size ${targetSize}! going with cell #${idx} with options ${Array.from(st[idx])}`);
    } else {
        console.log('no suitable cells found!');
        return st;
    }

    let options = Array.from(st[idx]);

    for (let option of options) {
        const newSt = st.slice();
        newSt[idx] = option;
        const res = isValid(newSt, constraints);
        console.log(`trying setting it to ${option}... ${res}`);
        if (!res) st[idx].delete(option);
    }
    if (st[idx].size === 1) {
        st[idx] = Array.from(st[idx])[0];
        console.log(`cell #${idx} has only one option: ${st[idx]}!`);
    } else {
        console.log(`cell #${idx} has now ${Array.from(st[idx])} options (${st[idx].size})`);
    }

    //if (stepNo > 4000) { return st; }

    lastChoices.push(idx);
    if (lastChoices.length > 10) lastChoices.shift();

    return () => solveStep(maxTargetSize, constraints, st, lastChoices, stepNo + 1);
}

function solve40(st0) {
    // array of board cells: each is a number or a set of numbers (it is only a number if cell already determined)
    const st = st0.map(v => (v === undefined || v === null) ? get8() : v);
    return trampoline(() => solveStep(8, allConstraints[40], st, [], 0));
}

function solve60(st0) {
    // array of board cells: each is a number or a set of numbers (it is only a number if cell already determined)
    const st = st0.map(v => (v === undefined || v === null) ? get10() : v);
    return trampoline(() => solveStep(10, allConstraints[60], st, [], 0));
}

// 2026-01-11 easy id: bc3a408847c14afdfc367af979779c3fe88ef7e65113ea47309f3835a2489197
console.log( solve40([1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]) )
//localStorage.setItem('SDKTP40', '[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]')
//localStorage.setItem('SDKTP40', '[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]')

// 2026-01-11 medium id: cc37840fd2238b48effb294b91fca577e9b9d88cbe6e3ea8afeff4ce4dacc110
//console.log( solve60([null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]) )
// localStorage.setItem('SDKTP60', '[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]')
// localStorage.setItem('SDKTP60', '')