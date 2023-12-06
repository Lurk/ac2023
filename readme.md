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
