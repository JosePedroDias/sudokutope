# Sudokutope

## TL;DR

based on https://sudokutope.com/

sudoku-like puzzle:

- 8 different symbols
- non orthogonal grid (a zonohedron, possibly a rhombicuboctahedron)
- grid has 40 cells
- the constraints per cell are slightly different:
    - there are 5 symmetrical petals, with 8 cells each - on each petal, all the 8 symbols must appear once
    - there are N "arms", with 8 cells each - on each arm, all the 8 symbols must appear once (how many?)


## puzzles

```js
copy(localStorage.getItem('SDKTP'))
localStorage.setItem('SDKTP', 'REPLACE_ME')
```


### easy

```json
[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]
```
