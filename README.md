# CSE291 Group Randomizer

## Build description

This simple program is written in rust. To compile, you can run the following command.

```sh
cargo build -r
```

## Using the randomizer

1. Prepare the input file. In the input file, each line contains each group member separated by ", " (This has a space after ","). Please sort member names by last name and first name manually to make sure that it always give the same order (Sorry, I'm just lazy to implement it).

For example, this is a valid input file for 2 groups.

```
Aaa Aaa, John Doe
Jane Doe, Joe Dohn, Zzz Xyz
```

2. Run the order randomizer

```sh
randomizer /path/to/your/input/file
```

## How it work?

For each line that represents a group, we compute Sha3 256 hash. Then, we multiply each byte by 1200077 (I just randomly pick some prime number here) and sum them up to an unsigned int64 group seed. After that, the group priority is generated by Isaac random generator using the group seed. Finally, we sort the group by group seed in ascending order.