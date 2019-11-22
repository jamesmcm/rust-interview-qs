# Number to Words problem

LeetCode: https://leetcode.com/problems/integer-to-english-words/

## Description

Convert a non-negative integer to its English words representation. 

Assume input is guaranteed to be less than 2^31 - 1.

## Provided test cases

### Test 1

#### Input

123

#### Output

"One Hundred Twenty Three"

### Test 2

#### Input

12345

#### Output

"Twelve Thousand Three Hundred Forty Five"

### Test 3

#### Input

1234567

#### Output

"One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"

### Test 4

#### Input

1234567891

#### Output

"One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

## Solution

The idea of the solution implemented here is as follows:

* Split the number in to groups of thousands and process the groups individually.
* Work backwards from the smallest group to the largest.
* Have fixed lookups for numbers less than 20 and the multiples of ten < 100.

