# Number of Islands problem

LeetCode: https://leetcode.com/problems/number-of-islands/

## Description

Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

## Provided test cases

### Test 1

#### Input

```
11110
11010
11000
00000
```

#### Output

1

### Test 2

#### Input

```
11000
11000
00100
00011
```

#### Output

3

## Solution

The idea of the solution implemented here is as follows:

* Scan through the 2D Vector of characters
* If we are on Land ('1'), and this Position (i.e. 2D index, (0, 0)) has not yet been seen (i.e. not in Seen set) then:
** Apply the Flood Fill algorithm to the current position.
** Increment the num\_islands counter.
* Flood Fill will add the Positions of all the connected Land sites to the Seen set, including the starting position (but never Water ('0')).
* We continue until the of the 2D Vector, and return num\_islands when we have checked all Positions.

We use the recursive Flood Fill implementation created in [Flood Fill](https://github.com/jamesmcm/rust-interview-qs/tree/master/flood_fill).

### Performance

* The recursive solution will fail for large Vectors as we exceed the stack limit with our stack frames due to the large number of recursive calls. 
* Rust does not support tail call recursion to avoid this issue (the compiler could convert the recursion to a loop under the hood, if the recursive call is the final operation in the function).
* Can we avoid visiting every element in the 2D Char Vector?
** i.e. what if there was one huge land mass, and we still check that all the elements are in Seen, one-by-one?
** What if we have a large sea mass - can we use flood fill on the sea to avoid checking all of those elements one-by-one?
** A new island can never be directly adjacent to a previous island, so we can skip those Positions.


