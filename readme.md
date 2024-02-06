# Advent of Code 2023 (code/notes)

Installation:

```shell
cargo install --git https://github.com/Lurk/ac2023.git ac2023
```

run:

```shell
ac2023 <day first|second...fifteenth> <path to input> <part number one|two>
```


## Day 1

`Off by one` errors everywhere. 

## Day 2

Today's calculation will have two parts: `one` and `two`. 

Initial idea of using `PartialOrd` to compare Round's was bad. 

## Day 3

IDK if I should support all previous days. Does not seem like a lot of work.

Today's task is much more complex than previous ones. Tests are required. 

There is a huge potential to optimize, but it is Sunday, and I still haven't finished the first Fallout.

## Day 4

Part one was surprisingly easy. 

Part two was hard to understand, probably because of fever.

## Day 5

Brute force approach takes too much time.

`rayon` goes brrrr

Test case for part two passes but answer is incorrect. I am sad panda. 

Oooooh. The range in maps are not inclusive. So I passed first stage and test assignment by luck :)

## Day 6

That was easier than day 1

## Day 7

The day when my neat little abstraction broke on part two, and now it is not as beautiful as it can be.

## Day 8

Today is the day when brute force is no go. 

After couple of hours looking at the debug output - we are looping with the same intervals. Which means what we need 
to do is found all loop lengths, and then find LCM of those. 

My math is rusty. Lets do naive LCM.

Result in 3m is ok. I guess.

I could not relax, so I googled how people calculate LCM. It turned out that more than 2000 years ago, our boy Euclid 
had already [figured it all out](https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm).

Now it is `blazingly`(TM) fast. 

## Day 9

To solve today's riddle, I wrote the least amount of code.

## Day 10

For some reason instead of following the trail I was trying to clear the map from unconnected pipes. Spent a few hours 
before I realized that there is easier way to solve part one. 

Part two. 

Ok, first idea, since we have our loop of pipes from part one. Lets replace all tiles that are not connected to main
loop with zeroes. 

I think I can simplify the loop itself by shortening 180 turns. For example this transformation: 

```
0000
0F70
0||0
```

Can be:

```
0000
0..0
0F70
```
Without loosing information.

That was bad idea. I spent too much time implementing it before realizing that it does not lead me anywhere. Better 
one is to follow the loop, and mark left/right. One of the sides will be inside and the other outside. Fill the void 
and count. 

End result is ugly as hell, but gives correct result. 

Added some love to it before starting next day. Also the way I did pipes sucks. 

```rust
Tile::Pipe(Direction::North, Direction::East)
```
That's an `L` pipe. Yeah, I know. Not obvious at all. 

## Day 11

Somehow columns are hard. Spent too much time implementing universe expansion. Everything else was kind of easy.

Part two does not fit into memory. Even if I make it fit with `bitvec` or similar bit manipulating library the amount 
of data to process will be too big to process in sane time. 

Current idea is to expand the universe few times. Find the speed at which galaxies are moving away from each other. 
Knowing the speed end distance should be easy to find.

I am already lagging one day behind, so the best move right now is go to sleep.

That Idea about speed was bad. I spend too much time figuring out how to measure rate of expansion, and none of my 
measurements made sense. Anyway, working idea - instead of actually expanding the universe you can gather position of 
galaxies andjust manipulate those positions. 

Will not rewrite the first part. 

## Day 12

TBH, do not know where to start. Will start with a nap.

Part one was hard. I had no clue where to start until i saw this post: 
[Solving Nonograms with 120 Lines of Code](https://towardsdatascience.com/solving-nonograms-with-120-lines-of-code-a7c6e0f627e4).

Again. Tests are green, but result is wrong :(

I gave up and looked at reddit. This was the test case that I was missing `??#???##?? 1,3`

I did not solve part two with a brute force. Despite spending 46 nanosecond on checking each combination, some of the
rows had so many of them that solving them require a few hours. 

With a Christmas party and hangover after it I am lagging five days behind :(

Ok. Instead of generating all possible combinations I can use a tree. `??#???##?? 1,3` can be 


```
                            _/\_
                          0|0  0|0
                    -------.    #------             1) '.', 2) '#' - 1,2 are possible
                  1|0     1|0  1|1    1|1 
                   .       #    .      #            1) '..', 2) '.#', 3) '#.', 4) '##' - 1,2,3 are possible  
                  2|0     2|1  2|1     
               ----#--     #  --#------             1) '..#', 2) '.##', 3) '#.#'  - 1,3 are possible 
              3|1    3|1     3|1      3|1
          -----.----  #       .   -----#---         1) '..#.', 2) '..##', 3) '#.#.', 4) '#.##' - 1,4 are not possible
         4|1      4|1            4|1     4|1
        --.----    #-------       .     --#----     1) '..#..', 2) '..#.#', 3) '#.##.', 4) '#.###' - 1,2,4 are possible
       5|1   5|1  5|1     5|1          5|1    5|1
        .     #    .       #            .      #    1) '..#...', 2) '..#..#', 3) '..#.#.', 4) '..#.##', 5) '#.###.', 
       6|1   6|1          6|1          6|1              6) '#.####' - 1,2,4 are possible
        #     #            #            #           1) '..#...#', 2) '..#..##', 3) '..#.###', 4) '#.###.#' 
       7|1   7|1          7|1                           - 1,2,3 are possible
  ------#     #-------     #                        1) '..#...##', 2) '..#..###', 3) '..#..####' - 1,2 are possible
 8|1   8|1   8|1    8|1   
  .  ---#-    .      #                              1) '..#...##.', 2) '..#...###', 3) '..#..###.', 4) '..#..####'
    9|1 9|1  9|1                                        - 2,3 - are possible
     .   #    .                                     1) '..#...###.', 2) '..#...####', 3) '..#..###..' 
     ^        ^                                         - 1,3 are possible
```
Where (spring index) | (group index)

Now, while I can eyeball what is possible and what is not, somehow I can not think of way to solve it elegantly :(

Yeah, so, few hours and bunch of ifs later, tests are passing. First part is producing the same result as before. 
Everything is super fast. But, result is `too low`. 

Added more tests. They are also green. Result is still wrong. Maybe it is time to give up and go to reddit. 

All test cases that I found on reddit also green. 

I am giving up. Can't come up with a test case that would fail. Will go and do other tasks.

## Day 13

Part one was fairly easy. Which had a healing effect for my ego after previous day failure. 

The main struggle in the second part was to get what I actually need to do. But I managed.

## Day 14

Part one is solved by splitting and sorting.

I knew that brute force will not work. Tried anyway, and even small test case did not finished in reasonable time. 
Debug output shows that after some amount of iterations results are cycling. Current plan is:

* cycle detection
* cycle length/start extraction 

Easy.

TBH I did not expected it to be easy. But it was.

## Day 15

Part one is surprisingly easy.

Everyday I fear that it will be day 12 all over again. But not this day.

## Day 16

Since I already have stuff like `Map` and `Direction` from previous days, moving the light beam was easy. Only 
complication was the fact that there is a lot of movement and you need to remember from which tile to which direction 
you already moved.  

Also today I managed to do three parts before going to sleep. Kind of proud of myself :)

Part two was even easier. I thought that I will need to use some black magic to speed things up, but everything was 
fast enough already.

## Day 17

Off course I can google `Dijkstra's algorithm`, but it would be too easy. 

I am so proud of myself. My homegrown algorithm does find shortest path on test data only in 10 minutes :) And dies
with stack overflow on the real data. Maybe it is time to call Dijkstra for help.

Spent some time reading [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) and can't figure out 
how to apply the no more that 3 steps in one direction rule to it. But reading it gave me idea that I want to try. 

For every point point the map we have a few directions we can go. The amount of those directions depends on 3 previous 
ones. Which means that for every pair of `([Option<Direction>;3], index)` we can remember min distance. 

That work very well. Test run finishes in 0.04s, which is much better than 10 minutes. Run on a real data still fails 
with stack overflow. 

`LLDB` shows that Iterators are occupying a lot of space on stack. Replaced them with for loop. Still not good enough. 
The question is should I continue look for a way to optimize amount of branches, or should I look for a completely new 
approach.

Ok. The way to reduce branches is to pass a reference to current best, and if current path produces greater value then 
there is no sense to continue. Algorithm gives a correct result on a test data, finishes on a real data with result 
`1044`,  but `That's not the right answer; your answer is too high. Curiously, it's the right answer for someone else; 
you might be logged in to the wrong account or just unlucky`. Yes. That's who I am. Unlucky :)

In the current implementation I am getting the `current_best` from path only when algorithm on a last tile. Because of 
caching, that happens only a few initial times. At the same time cache contains the missing part of calculation. I can 
improve that part while thinking how to fix actual error.

While updating that thing I have one more idea regarding `current_best` instead of `usize::MAX` I can use worst case 
which is longest path times highest tile value. Where longest past is length of `(side 1 + side 2) * 9`. 

That made test fail. Which make me thinking that `current_best` and caching are preventing from visiting some of the
tiles. Without cache everything is too slow, so I am back to the same question - should I continue look for a way to 
optimize amount of branches, or should I look for a completely new approach. Looks like optimizing branches are not as 
easy as I hoped it would be. Time to get of the keyboard and think of a new approach. 

The best idea I come up so far is to unroll recursion. Will do that and see how it goes.

I am not smart enough to unroll that recursion. Instead ill try to do some kind of `Dijkstra's algorithm`. Here is how 
the rough idea looks like. Initialize visited vector with `None`. Mark start index as Some(0). For every `visited tile`
we get all `unvisited neighbors` with list of values from `visited tiles`. For every `unvisited neighbor` we get lowest 
value from the list and put sum of that value and value of current tile to visited vector. Lowest value at the end will 
be shortest distance.

First iteration gives close to true value, but not good enough. Best guess, when `unvisited neighbor` has `visited` 
ones that have the same value `11` and `11` for example, I do not know which one to choose, so I am choosing the first 
one. That will impact the route later because of the `rule of three` from the task.

So, let's zoom out a bit. 

```
1 2 3 4 5
2 2 3 4 5
3 2 3 4 5
4 2 3 4 5
5 2 3 4 5
```

Consider how I can get to the tile (3,3):

- (0,3) E (1,3) E (2,3) E (3,3) from E with 3 E
- (1,2) S (1,3) E (2,3) E (3,3) from E with 2 E
- (2,1) S (2,2) S (2,3) E (3,3) from E with 1 E
- (3,0) S (3,1) S (3,2) S (3,3) from S with 3 S
- ...

Which means my node look like:
`(x,y, direction_from_where_we_got_here, amount_of_steps_to_the_same_direction) -> distance`. It took me some time to 
make it work. But in the end I have two steps. First - building a graph. Second - implementing good enough 
`Dijkstra algorithm` to find a shortest path on that graph. Can be better. But it is good enough already.

Part two required to build slightly different graph. Which was hard because of a new year celebration.

Happy new year BTW.

## Day 18

Since I already built most of the moving parts (Map, Directions) part one was relatively easy to build.

Current map expansion is way to slow for the part two. 

Ok. I made it faster. Now I am running out of memory. Even if I use `u8` instead of `char` as my tile, it still tries 
to allocate more that 150 gigs of ram. 

 There should be formula to calculate area of polygon. 

Turned out there is [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) with straightforward 
implementation. 

The answer is `too low`. Hmmm.

It gives almost expected result on a test data:

- 952404941483
- 952408144115

I suspect it is because Shoelace formula does not account for the 'trench' which should be our perimeter. 

No. It is too big. 

Can it be half of it? It is. Almost. I don't know why (too sleepy to figure it out) but the difference between Shoelace
formula result and expected area is `perimeter / 2 + 1`. 

## Day 19

Part one has complicated rules to encode, and maybe I over engineered it a bit.

Same is true for the part two. Even after I wrapped my head around what needs to be done, it was hard to keep all 
requirements in mind. Right naming helps a lot. 

## Day 20

It is this kind of day, tests are green but result on a real data is `too low`. 

I don't see any obvious error and too sleepy to continue looking. 

Yep. That was bug in parsing. Lets see what's in the part two.

Part two always starts with "I can brute force through it". Didn't work this time.

While looking at various debug output, i noticed a pattern:

```
button press# 3778, origin: 7, value: High
button press# 3888, origin: 28, value: High
button press# 3906, origin: 53, value: High
button press# 4050, origin: 38, value: High
button press# 7557, origin: 7, value: High
button press# 7777, origin: 28, value: High
button press# 7813, origin: 53, value: High
button press# 8101, origin: 38, value: High
button press# 11336, origin: 7, value: High
```

I need all of those origins to be High at the same button press. Which looks like task for the `LCM`. Will check it out 
tomorrow.

Yeah. I was on a right track yesterday. The answer is an `LCM` of button presses amount, with one caveat. The count 
should start from 1 not from 0.

# Day 21

First part was very easy to brute force. Just flip tiles on the map and then count the values. 

In part two it was obvious from the beginning that growing map will not fit to the memory. First Idea. I will have a 
queue where I will push back next positions on the map, and pop front positions that I need to calculate on the current 
step.

Hm. It gives more positions that I need. Oh. Oh. One point gives four possible points on the next step. Each of those
points on the next step will get back to previous point. And I need only one of those. Can't figure out nothing smarter 
than a HashSet.

It takes too long to compute 1000 steps on a test data. I need to figure out if there is a repeating pattern in the 
result. The result is growing each step. The rate of growth is also growing. What about that rate?

There is a pattern. On a test data, starting from step 39, second derivative is `2` every eleventh step. 

```
39: 944 d: 50, d2: 2
40: 989 d: 45, d2: -5
41: 1053 d: 64, d2: 19
42: 1107 d: 54, d2: -10
43: 1146 d: 39, d2: -15
44: 1196 d: 50, d2: 11
45: 1256 d: 60, d2: 10
46: 1324 d: 68, d2: 8
47: 1383 d: 59, d2: -9
48: 1464 d: 81, d2: 22
49: 1528 d: 64, d2: -17
50: 1594 d: 66, d2: 2
51: 1653 d: 59, d2: -7
52: 1735 d: 82, d2: 23
53: 1805 d: 70, d2: -12
54: 1853 d: 48, d2: -22
55: 1914 d: 61, d2: 13
56: 1988 d: 74, d2: 13
57: 2072 d: 84, d2: 10
58: 2145 d: 73, d2: -11
59: 2244 d: 99, d2: 26
60: 2324 d: 80, d2: -19
61: 2406 d: 82, d2: 2
62: 2479 d: 73, d2: -9
63: 2579 d: 100, d2: 27
64: 2665 d: 86, d2: -14
65: 2722 d: 57, d2: -29
66: 2794 d: 72, d2: 15
67: 2882 d: 88, d2: 16
68: 2982 d: 100, d2: 12
69: 3069 d: 87, d2: -13
70: 3186 d: 117, d2: 30
71: 3282 d: 96, d2: -21
72: 3380 d: 98, d2: 2
```

And while all other values are all over the place the sum of second derivatives of those ten steps is always 16. There 
is two problems though. While I can eyeball it, I can't come up with a way to detect such a pattern, and most 
importantly - I have no clue how that can help me to solve my problem. Yet. 

Map from test input happens to be square and eleven is the length of a side.

Before I start to think how that can help me and then how to detect such a pattern, I want to check if such pattern 
even exists on real input.

I can not eyeball it on 1000 steps. After adding grouping by second derivative the pattern is clearly visible there.
Starting from step 132 `0` repeats every 131 step. 131 is length of a side in the real input. Looks like this is the 
way to to detect a pattern. Get a side length, iterate through the steps until there will be repeating second 
derivative.

Let's figure out what to do with it tomorrow.

That was a long time since I wrote previous sentence. I got sick, and was in bed without any energy to continue. Now, I
hope, I am back to presick performance levels. Lets figure out whats left. 

131 does not help at all. 26501365 - is amount of steps. It is not divisible by 131. 26501365 - 133 is also not 
divisible by 131. So yeah, while there is clearly a pattern, I do not see how it can help me.

Was playing a bit with numbers. IDK if that has a meaning but *2023*00 * 131 + 65 = 26501365

```
65 + 131 * 0 = 3_917
65 + 131 * 1 = 34_920
65 + 131 * 2 = 96_829
65 + 131 * 3 = 189_644
65 + 131 * 4 = 313_365
65 + 131 * 5 = 467_992
```

Does it make any sense?

It does. Oh boy it does. d1 - first derivative. d2 - second derivative.

```
N     Value       d1         d2
0    3_917
1    34_920      31_003
2    96_829      61_909     30_906
3    189_644     92_815     30_906           
```

Which means I can use simple math to calculate needed value like that

```
1: 3_917 + 31_003 + 30_906 * 0 = 34_920
2: 34_920 + 31_003 + 30_906 * 1 = 96_829
3: 96_829 + 31_003 + 30_906 * 2 = 189_644
4: 189_644 + 31_003 + 30_906 * 3 = 313_365
5: 313_365 + 31_003 + 30_906 * 4 = 467_992
```

Now I need to repeat that 202300  times. Easy. 

## Day 22

The first part of the first task looks like 3D Tetris. 

I am pretty sure that there is math that answers the question if two segments intersect each other.

