# Hexagonal Grid Problem
Solution to the hexagonal grid challenge problem.

## Overview

This originated from an interview question that was posed to me many years ago:
imagine you are looking at a finite grid of hexagonal tiles, where each tile is represented by a letter.
Given a sequence of letters (e.g. "ABCDEFG"), write a program that can determine if it is possible
to traverse the grid, starting at the tile represented by the first letter in the sequence, and moving
only to adjacent tiles for each subsequent letter. In other words, is each letter in the grid adjacent
to the letters before and after it in the sequence?

(Note: for the purposes of the interview question, it was not necessary to actually produce the route used.
However, I found it more interesting to find the route anyway. Doubtless there is a more efficient solution
that only checks if the letters are adjacent).

This problem occupied me on and off for the better part of 10 years until I decided to use it as an opportunity
to learn Rust. I don't know if it is a coincedence that I was only able to fully solve this problem

## Detailed Explanation of Solution

The hardest part of this problem is undoubtedly trying to figure out which tiles are adjacent to each other.

The grid is defined as such: the input to the program is the sequence of letters that would be produced if
you started at the center of the grid with the first letter, moved out one space, then spiraled clockwise around and outward
from the center.

In other words the grid follows this pattern:

```
    18   7   8
  17   6   1   9
16   5   0   2   10
  15   4   3   11
    14   13  12
```

For example, for the input sequence "ABCDEFGHIJKLMNOPQRS" the grid would look like this:

```
    S   H   I
  R   G   B   J
Q   F   A   C   K
  P   E   D   L
    O   N   M
```

Perhaps there is some mathematical formula to determine which indices are adjacent to others, but if 
there is, I'm not smart enough to figure it out. That just leaves trying to represent this grid in
memory somehow. Computers are great at representing linear data - after all, all memory in a computer
is just stored in a linear array. We can trick computers into representing rectangular data using 
nested arrays and clever manipulation of the indices (e.g. `n = y * w + x`). This extends to any
n-dimensional rectangular object (cubes, hypercubes, etc.).

However, computers are not naturally adept at representing hexagonal objects. I played around with
a few different representations: a linked-grid of nodes which each had 6 neighbor pointers, a complex
x/y/z index system, and several other bad ideas.

The solution I finally settled on was more obvious that I would like to admit: just squeeze the hexagonal
grid into a rectangular grid! In other words, rearrange the grid into something that will fit into a 2d
array and define a set of rules about which (x, y) coordinates are "adjacent" to each other.

For example, let's take the above grid and rotate it a bit:

```
Q   R   S
P   F   G   H
O   E   A   B   I
    N   D   C   J
        M   L   K
```

(This might look slightly rotated from what you expect, but I promise the solution works out the same.)

This can easily fit into a 2d array, even though there is some empty space in the corners. For large inputs
this could lead to a lot of wasted memory, and there are ways around that, but for demonstration purposes it's
easier to view it like this.

Now we have to determine which letters are adjacent to each other. Looking back at the grid, we can take for
example the letter A and see that it's neighbors are: B, C, D, E, F, G. In our rotated 2d array we can see that
this corresponds to the entries that are immediately up, down, left, and right, as well as down-right and up-left.

As it turns out, this is true for any tile in the grid - it's neighbors will always be at the coordinates represented
by these offsets. In (x, y) terms these are represented by:
* (x + 1, y) (right)
* (x - 1, y) (left)
* (x, y + 1) (up)
* (x, y - 1) (down)
* (x + 1, y - 1) (down-right)
* (x - 1, y + 1) (up-left)

And there you have it! Given any (x, y) coordinate in the 2d array you can easily determine which letters are adjacent
to it in the hexagonal grid. Now all that is left to do is find the starting point by scanning over the array for the
matching letter, and then checking it's neighbors to see if any match the next letter in the sequence, repeating until
the sequence ends or no neighbors match.

There is more complexity in producing the final answer than this (for example it is non-trivial to actually construct
the grid given an input sequence and spiraling out from the center), but I will let the code speak for itself from
here on.

## TODO
* Allow for configurable grid (currently it is locked to the example shown above).
* Take grid pattern and route from command-line arguments.
* More thorough input and error checking.
* Cleaning up / commenting code.
