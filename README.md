# Renghlish

## Introduction

English is boring and so is the Latin writing system.

```text
      │╭────╮  │  │ ·   │       ·         │╭──         ┌─╮·    │ · ──╮
     ─┼┼────┼──┼──┼─────┼      ───       ─┼┼──         │───────┼─────┼
      │╰──  │  │  │ ╭   │       ╭         │╰── ╭       │  ·    │     │
      │┌─┐──┤╭─╯  │─┼─╭─╯      ─┼─        │─┼──┼─      │       │┌─┐──┤
    ╭─╯└─┘  ││  ──╯─╯ │        ─╯       ──╯─╯ ─╯       │     ╭─╯└─┘  │
    │     ──┴┴──      ┴──                              ┴──   │     ──┴
```

Renghlish is less boring!

> Interested? Read more on my [blog](https://davnotdev.github.io/blog/interesting/renghlish/)!

This library simply converts your lame Latin characters into Renghlish.

## Usage

```rust
use renghlish::*;

fn main() {
    assert_eq(words("No special symbols!"), None);

    word("hello")
        .unwrap()
        .get_rows()
        .iter()
        .for_each(|row| eprintln!("{row}"));

    words("Hello World")
        .unwrap()
        .get_rows()
        .iter()
        .for_each(|row| eprintln!("{row}"));

    let word_buf = WordBuffer::default();
    word_buf.vowel(vowels::vowel_qu());
    word_buf.vowel(vowels::vowel_a());
    word_buf.constanant(roots::root_c());
    word_buf.constanant(roots::root_k());
    word_buf
        .get_rows()
        .iter()
        .for_each(|row| eprintln!("{row}"));
}

```

## That's all

```text
      │──╮╭──      ──╮      ──╮·  ·           ──╮  │
     ─┼──┼┼──      ──┼       ─┼──────       ────┼──┼
      │──╯╰──      ──╯        │·  ·╮        ╮ ──╯  │
    ╭─╯                     ──┤   ─┼─      ─┼─   ──┤
    │  ╲╱                     │    ├─       ├─     │
    ┴──                     ──┴    ╰╴       ╰╴     │
```
