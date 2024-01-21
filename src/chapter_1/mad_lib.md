# Mad Lib

## Challenge

> Generates random stories from tuples of (noun, verb, adjective, adverb).

## Solution

To generate stories, we can start with a template and fill in the blanks. For instance, when provided
with the template

```rust,noplayground
template = "Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!";
```
and the values `noun="dog"`, `verb="walk"`, `adjective="blue"`, `adverb="quickly"`, the resulting
story is

```
Do you walk your blue dog quickly? That's hilarious!
```

The missing pieces are the lists of words and a method for selecting words from them.
Here are my lists of not-so-funny-but-safe words:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/mad_lib.rs:database}}
```

Below is a function for drawing a random word from a list.
Notice the parameter `'a` indicating the lifetime of the borrowed return value.

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/mad_lib.rs:fn_random_choice}}
```

With all the elements in place, we can wire up the final solution:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/mad_lib.rs:main}}
```

Some sample outputs:

```
Do you touch your wet elephant quickly? That's hilarious!
Do you feed your cool cat hungrily? That's hilarious!
Do you walk your soft goldfish quickly? That's hilarious!
```

With the presented word lists, how many distinct stories are there?
According to the [rule of product](https://en.wikipedia.org/wiki/Rule_of_product), the answer is

$$
|nouns| \times |verbs| \times |adjectives| \times |adverbs|
= 5 \times 5 \times 7 \times 4 \
= 700
$$

where `|S|` denotes the number of elements of in a set `S`.
