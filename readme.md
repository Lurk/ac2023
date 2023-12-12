# Advent of Code 2023 (code/notes)

Installation:

```shell
cargo install --git https://github.com/Lurk/ac2023.git ac2023
```

## Day 1

`Off by one` errors everywhere. 

```shell
ac2023 first <path to input> one
```

## Day 2

Today's calculation will have two parts: `one` and `two`. 

Initial idea of using `PartialOrd` to compare Round's was bad. 


```shell
ac2023 second <path to input> one
```

```shell
ac2023 second <path to input> two
```

## Day 3

IDK if I should support all previous days. Does not seem like a lot of work.

Today's task is much more complex than previous ones. Tests are required. 

There is a huge potential to optimize, but it is Sunday, and I still haven't finished the first Fallout.


```shell
ac2023 third <path to input> one
```

```shell
ac2023 third <path to input> two
```

## Day 4

Part one was surprisingly easy. 

Part two was hard to understand, probably because of fever.

```shell
ac2023 fouth <path to input> one
```

```shell
ac2023 fourth <path to input> two
```

## Day 5

Brute force approach takes too much time.

`rayon` goes brrrr

Test case for part two passes but answer is incorrect. I am sad panda. 

Oooooh. The range in maps are not inclusive. So I passed first stage and test assignment by luck :)

```shell
ac2023 fifth <path to input> one
```

```shell
ac2023 fifth <path to input> two
```

## Day 6

That was easier than day 1

```shell
ac2023 sixth <path to input> one
```

```shell
ac2023 sixth <path to input> two
```

## Day 7

The day when my neat little abstraction broke on part two, and now it is not as beautiful as it can be.

```shell
ac2023 seventh <path to input> one
```

```shell
ac2023 seventh <path to input> two
```

## Day 8

Today is the day when brute force is no go. 

After couple of hours looking at the debug output - we are looping with the same intervals. Which means what we need 
to do is found all loop lengths, and then find LCM of those. 

My math is rusty. Lets do naive LCM.

Result in 3m is ok. I guess.

I could not relax, so I googled how people calculate LCM. It turned out that more than 2000 years ago, our boy Euclid 
had already [figured it all out](https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm).

Now it is `blazingly`(TM) fast. 

```shell
ac2023 eighth <path to input> one
```

```shell
ac2023 eighth <path to input> two
```

## Day 9

To solve today's riddle, I wrote the least amount of code.

```shell
ac2023 ninth <path to input> one
```

```shell
ac2023 ninth <path to input> two
```

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

```shell
ac2023 tenth <path to input> one
```

```shell
ac2023 tenth <path to input> two
```

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


