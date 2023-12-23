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
galaxies and just manipulate those positions. 

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


