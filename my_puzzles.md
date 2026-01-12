# 40

`cargo run --release --bin rustsolver solve 40`

## we get a fully solved puzzle
`[2,4,5,1,4,1,8,7,7,6,2,3,8,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,4,8,5,7,5,2,1,6,3,8,4]`

`cargo run --release --bin rustsolver add_gaps '[2,4,5,1,4,1,8,7,7,6,2,3,8,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,4,8,5,7,5,2,1,6,3,8,4]' 30`

## get get multiple puzzles with increasing difficulty as more gaps are introduced
```
  Gaps:  1 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,1,4,1,8,7,7,6,2,3,8,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,4,8,5,null,5,2,1,6,3,8,4]
  Gaps:  2 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,6,2,3,8,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,4,8,5,null,5,2,1,6,3,8,4]
  Gaps:  3 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,6,2,3,8,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,null,8,5,null,5,2,1,6,3,8,4]
  Gaps:  4 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,6,2,3,null,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,null,8,5,null,5,2,1,6,3,8,4]
  Gaps:  5 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,null,8,5,null,5,2,1,6,3,8,4]
  Gaps:  6 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,6,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps:  7 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,2,4,5,5,8,3,1,6,7,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps:  8 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,2,4,5,5,8,3,1,6,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps:  9 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,2,4,5,5,8,null,1,6,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 10 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,2,4,5,5,8,null,1,null,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 11 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,null,4,5,5,8,null,1,null,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 12 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,null,null,5,5,8,null,1,null,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 13 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,null,null,5,5,8,null,null,null,null,7,6,3,1,2,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 14 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,3,null,null,3,null,null,5,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 15 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,null,null,null,3,null,null,5,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,1,6,3,8,4]
  Gaps: 16 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,null,null,null,3,null,null,5,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,null,6,3,8,4]
  Gaps: 17 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,null,null,null,3,null,null,5,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,null,6,3,8,null]
  Gaps: 18 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,7,null,2,null,null,null,3,null,null,null,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,null,6,3,8,null]
  Gaps: 19 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,null,null,2,null,null,null,3,null,null,null,5,8,null,null,null,null,7,6,3,1,null,null,8,5,null,null,2,null,6,3,8,null]
  Gaps: 20 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,null,null,2,null,null,null,3,null,null,null,5,8,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null]
  Gaps: 21 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,null,null,2,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null]
  Gaps: 22 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,1,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null]
  Gaps: 23 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null]
  Gaps: 24 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,2,null,6,3,8,null]
  Gaps: 25 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,6,3,8,null]
  Gaps: 26 | Difficulty:   0 total, 0.00 avg,  0 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,6,3,null,null]
  Gaps: 27 | Difficulty:  85 total, 3.70 avg,  7 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,null,3,null,null]
  Gaps: 28 | Difficulty:  90 total, 3.75 avg,  7 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,null,null,null,null,null,null,3,null,null]
  Gaps: 29 | Difficulty: 113 total, 4.19 avg,  7 max | [2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,null,6,3,null,null,null,null,null,null,null,null,null,null,3,null,null]
  Gaps: 30 | Difficulty: 128 total, 4.57 avg,  7 max | [2,null,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,null,6,3,null,null,null,null,null,null,null,null,null,null,3,null,null]
```

# 60

`cargo run --release --bin rustsolver solve 60`

## we get a fully solved puzzle
`[1,0,2,1,4,9,5,7,7,0,9,3,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,8,4,9,1,3,6,5,7,7,2,6,9,1,4,3,8,0,5]`


`cargo run --release --bin rustsolver add_gaps '[1,0,2,1,4,9,5,7,7,0,9,3,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,8,4,9,1,3,6,5,7,7,2,6,9,1,4,3,8,0,5]' 40`

## get get multiple puzzles with increasing difficulty as more gaps are introduced
```
  Gaps:  1 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,3,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,8,4,9,1,3,6,5,7,7,2,6,9,1,4,3,8,0,5]
  Gaps:  2 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,3,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,8,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  3 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,8,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  4 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,3,9,4,8,2,2,0,null,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  5 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,5,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,4,8,2,2,0,null,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  6 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,2,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,4,8,2,2,0,null,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  7 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,4,8,2,2,0,null,4,9,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  8 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,4,8,2,2,0,null,4,null,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps:  9 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,2,null,9,1,4,3,8,0,5]
  Gaps: 10 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,5,7,7,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,2,null,null,1,4,3,8,0,5]
  Gaps: 11 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,7,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,2,null,null,1,4,3,8,0,5]
  Gaps: 12 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,2,null,null,1,4,3,8,0,5]
  Gaps: 13 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,3,8,0,5]
  Gaps: 14 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,8,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 15 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,2,1,null,9,5,7,7,0,9,null,null,3,1,null,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 16 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,7,0,9,null,null,3,1,null,6,null,2,5,8,1,4,6,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 17 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,7,0,9,null,null,3,1,null,6,null,2,5,8,1,4,null,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 18 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,6,null,2,5,8,1,4,null,6,8,3,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 19 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,6,null,2,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 20 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,null,null,2,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]
  Gaps: 21 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,null,null,2,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,null,0,5]
  Gaps: 22 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,null,null,null,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,null,0,5]
  Gaps: 23 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,null,null,null,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,null,4,null,null,0,5]
  Gaps: 24 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,null,4,null,null,0,5]
  Gaps: 25 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,null,4,null,null,0,5]
  Gaps: 26 | Difficulty:   0 total, 0.00 avg,  0 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,5]
  Gaps: 27 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,5]
  Gaps: 28 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 29 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 30 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,null,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 31 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,1,null,9,5,7,null,0,null,null,null,3,null,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 32 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,null,null,9,5,7,null,0,null,null,null,3,null,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 33 | Difficulty:   8 total, 2.00 avg,  2 max | [1,0,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 34 | Difficulty:  11 total, 2.20 avg,  3 max | [1,0,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,5,8,null,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 35 | Difficulty:  11 total, 2.20 avg,  3 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,5,8,null,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 36 | Difficulty:  11 total, 2.20 avg,  3 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 37 | Difficulty:  11 total, 2.20 avg,  3 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,8,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]
  Gaps: 38 | Difficulty:  34 total, 2.62 avg,  4 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,8,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,null,null,null,0,null]
  Gaps: 39 | Difficulty:  34 total, 2.62 avg,  4 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,null,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,null,null,null,0,null]
  Gaps: 40 | Difficulty:  45 total, 2.81 avg,  4 max | [1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,null,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,null,7,7,null,null,null,null,null,null,null,0,null]
```

## output to game

```js

/// 40 puzzle

// 20 gaps, diff 0
localStorage.setItem('SDKTP40', '[2,4,5,null,4,1,8,7,null,null,2,null,null,null,3,null,null,null,5,8,null,null,null,null,7,6,3,1,null,null,null,5,null,null,2,null,6,3,8,null]')

// 25 gaps, diff 0
localStorage.setItem('SDKTP40', '[2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,6,3,8,null]')

// 27 gaps, diff 85
localStorage.setItem('SDKTP40', '[2,4,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,7,6,3,null,null,null,null,5,null,null,null,null,null,3,null,null]')

// 30 gaps, diff 128
localStorage.setItem('SDKTP40', '[2,null,5,null,4,null,8,7,null,null,null,null,null,null,3,null,null,null,5,null,null,null,null,null,null,6,3,null,null,null,null,null,null,null,null,null,null,3,null,null]')

/// 60 puzzle

// 20 gaps, diff 0
localStorage.setItem('SDKTP60', '[1,0,null,1,null,9,5,7,null,0,9,null,null,3,1,null,null,null,2,5,8,1,4,null,6,8,null,4,9,0,null,7,null,0,6,null,9,null,8,2,2,0,null,4,null,1,3,6,5,7,7,null,null,null,1,4,null,8,0,5]')

// 27 gaps, diff 8
localStorage.setItem('SDKTP60', '[1,0,null,1,null,9,5,7,null,0,null,null,null,3,1,null,null,null,null,5,8,1,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,2,2,0,null,4,null,null,3,6,5,7,7,null,null,null,null,4,null,null,0,5]')

// 35 gaps, diff 11
localStorage.setItem('SDKTP60', '[1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,5,8,null,null,null,6,8,null,4,9,0,null,7,null,0,6,null,null,null,8,null,2,0,null,4,null,null,3,null,5,7,7,null,null,null,null,4,null,null,0,null]')

// 40 gaps, diff 45
localStorage.setItem('SDKTP60', '[1,null,null,null,null,9,5,null,null,0,null,null,null,3,null,null,null,null,null,null,8,null,null,null,6,null,null,4,9,0,null,7,null,null,6,null,null,null,8,null,2,0,null,4,null,null,3,null,null,7,7,null,null,null,null,null,null,null,0,null]')
```
