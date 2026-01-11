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

localStorage.setItem('SDKTP40', 'REPLACE_ME')
localStorage.setItem('SDKTP60', 'REPLACE_ME')
```

### easy (40)

```json
[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]
```

### medium (60)

```json
[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]
```
