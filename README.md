# Sudokutope

## TL;DR

based on https://sudokutope.com/

Sudoku-like puzzle with:

- 8 different symbols;
- non orthogonal grid (a zonohedron, possibly a rhombicuboctahedron?);
- grid has 40 cells;
- the constraints per cell are slightly different:
    - there are 5 symmetrical petals, with 8 cells each - on each petal, all the 8 symbols must appear once;
    - there are 10 "arms"/arcs, with 8 cells each - on each arm, all the 8 symbols must appear once.


## Artwork and data

- I started from the original site's runtime SVG and tweaked it into a workable one (inkscape + 2 nodejs scripts). grid5.svg is the board I use in the game.
- for anyone interested in the puzzles' constraints and geometry, check `data.json` and `index.js`'s `cellGroups`.

## why did I do this?

I tend to learn by doing. Found this board's geometry to be challenging to interpret, therefore
I both wanted to see how easily I could implement the constraints and help visualize them during puzzle solving.
I believe my highlighting approach to be more effective.

## puzzles

```js
copy(localStorage.getItem('SDKTP'))
localStorage.setItem('SDKTP', 'REPLACE_ME')
```

### easy

```json
[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]
```

(this puzzle is from the site's easy category. I haven't attempted to generate puzzles)
