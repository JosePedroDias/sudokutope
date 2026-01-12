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

- I started from the original site's runtime SVG and tweaked it into a workable one (inkscape + nodejs scripts). `grid40.svg` and `grid60.svg` are the boards I use in the game.
- puzzle geometries and constraints can be found in `data40.json` and `data60.json`.

## why did I do this?

I tend to learn by doing. Found this board's geometry to be challenging to interpret, therefore
I both wanted to see how easily I could implement the constraints and help visualize them during puzzle solving.
I believe my highlighting approach to be more effective.

(cells, gaps, difficulty)
- [play 40/20/0](https://josepedrodias.github.io/sudokutope/index.html#40%20%5B2,4,5,null,4,1,8,7,null,null,2,null,null,null,3,null,null,null,5,8,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null%5D)
- [play 40/27/85](https://josepedrodias.github.io/sudokutope/index.html#40%20%5B2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,null,3,null,null%5D)
- [play 60/40/45](https://josepedrodias.github.io/sudokutope/index.html#60%20%5B1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,5%5D)
- [play 60/40/45](https://josepedrodias.github.io/sudokutope/index.html#60%20%5B1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,null,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,null,7,7,null,null,null,null,null,null,null,0,null%5D)

## solver

I wrote a [simple solver in JS](solver.js) which worked but was slow.
I [ported it to rust](rustsolver) and introduced some optimizations, using augment.
