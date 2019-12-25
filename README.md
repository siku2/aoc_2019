# Advent of Code 2019

It's that time of year again, I guess, so here are my solutions to the puzzles!
They're written in [Rust](https://www.rust-lang.org/) *again* because it's been a year and I haven't used it ever since so I've forgotten almost all of it...

I also decided to write a little post for each day.
If you're interested, you can read them [here](https://siku2.io/aoc_2019/).

## Command-line Usage

Here's the help text of the CLI:

```
Advent of Code 2019 1.0
Simon Berger

USAGE:
    aoc_2019 [OPTIONS] [INPUT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <PART>     Set the day (Defaults to the current day)
    -p, --part <PART>    Which part of the day to solve [default: both]  [possible values: first, second, both]

ARGS:
    <INPUT>    Sets the input file to use [default: STDIN]
```

By default, it takes the input from the console and solves both parts of "today's" puzzle with it.

If you're reading this in the future the "today" part obviously no longer applies so you will have to provide the `day` argument.
