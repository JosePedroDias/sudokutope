# Sudokutope

## TL;DR

based on https://sudokutope.com/

Sudoku-like puzzle with:

- non-orthogonal grid (a zonohedron, possibly a rhombicuboctahedron?);

    - easy
        - 8 different symbols (0-7);
        - grid has 40 cells;
        - there are 5 symmetrical petals, with 8 cells each - on each petal, all the symbols must appear once;
        - there are 10 "arms"/arcs, with 8 cells each - on each arm, all the symbols must appear once;
    - medium
        - 10 different symbols (0-9);
        - grid has 60 cells;
        - there are 6 symmetrical petals, with 10 cells each - on each petal, all the symbols must appear once;
        - there are 12 "arms"/arcs, with 10 cells each - on each arm, all the symbols must appear once.


## Artwork and data

- I started from the original site's runtime SVG and tweaked it into a workable one (inkscape + 2 nodejs scripts). grid5.svg is the board I use in the game.
- for anyone interested in the puzzles' constraints and geometry, check `data.json` and `index.js`'s `cellGroups`.

## why did I do this?

I tend to learn by doing. Found this board's geometry to be challenging to interpret, therefore
I both wanted to see how easily I could implement the constraints and help visualize them during puzzle solving.
I believe my highlighting approach to be more effective.

## puzzles

(these puzzles are from the site. I haven't attempted to generate puzzles)

```js
copy(localStorage.getItem('SDKTP40'))
copy(localStorage.getItem('SDKTP60'))

// 2026-01-11 easy id: bc3a408847c14afdfc367af979779c3fe88ef7e65113ea47309f3835a2489197
localStorage.setItem('SDKTP40', '[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]')

// 2026-01-11 medium id: cc37840fd2238b48effb294b91fca577e9b9d88cbe6e3ea8afeff4ce4dacc110
localStorage.setItem('SDKTP60', '[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]')

function fromTheirs(arr) { return arr.reverse().map(v => v === -1 ? null : v + 1); }

// 2026-01-12 easy id: a8d920301dd84619aef3da1e8eb717ee14818511f92522e1d1899e52ebfb321a
localStorage.setItem('SDKTP40', JSON.stringify(fromTheirs([-1,-1,0,-1,-1,7,-1,1,-1,2,7,6,-1,0,-1,-1,-1,-1,4,-1,-1,3,-1,-1,1,-1,-1,2,-1,6,-1,-1,-1,-1,4,-1,4,-1,5,-1])));

// 2026-01-12 medium id: d990e37b5c70621a5102da1ba446363b91c43aceeb3e0815694013719f5eb829
localStorage.setItem('SDKTP60', JSON.stringify(fromTheirs([-1,2,-1,4,-1,-1,-1,3,0,-1,9,-1,7,-1,-1,5,-1,8,-1,-1,3,-1,2,-1,-1,-1,-1,9,8,1,3,-1,5,-1,9,-1,-1,-1,6,-1,3,1,-1,2,-1,-1,4,1,-1,5,-1,-1,-1,-1,-1,7,6,0,-1,-1])));

// 2026-01-12 expert id: dbc406a81c1632b4eb025b89842c93eba763601048413dd6d8d42f78554996f1
localStorage.setItem('SDKTP60', JSON.stringify(fromTheirs([-1,-1,3,6,-1,-1,-1,-1,-1,4,-1,-1,8,2,-1,-1,-1,-1,-1,-1,-1,4,-1,-1,-1,-1,8,-1,-1,-1,9,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,1,-1,-1,-1,-1,-1,7,-1,-1,-1,-1,0,-1,-1,-1,-1,-1,-1])));
```
