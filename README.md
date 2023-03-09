# Renghlish

## Introduction

English is boring and so is the Latin writing system.

```plaintext
  │╭────╮  │  │ ·   │     ·       │╭──       ┌─╮·    │ · ──╮
 ─┼┼────┼──┼──┼─────┼    ───     ─┼┼──       │───────┼─────┼
  │╰──  │  │  │ ╭   │     ╭       │╰── ╭     │  ·    │     │
  │┌─┐──┤╭─╯  │─┼─╭─╯    ─┼─      │─┼──┼─    │       │┌─┐──┤
╭─╯└─┘  ││  ──╯─╯ │      ─╯     ──╯─╯ ─╯     │     ╭─╯└─┘  │
│     ──┴┴──      ┴──                        ┴──   │     ──┴
```

Renghlish is less boring!

> Interested? Read more on my [blog](https://davnotdev.github.io/blog/interesting/renghlish/)!

The `renghlish` crate simply converts your lame Latin characters into Renghlish
unicode art.
The `eng2reng` binary can be used to just try it out.

## Try it Out

> Do these binaries actually work?
> Who knows.
> Look, I haven't tested these.

### Windows

You can [download the cli at here](https://github.com/davnotdev/renghlish/releases/download/v0.0.1/eng2reng_windows.exe).

```cmd
eng2reng_windows.exe Hello World
```

### Macos

```sh
# Fetch the file into /tmp/eng2reng.
curl -L https://github.com/davnotdev/renghlish/releases/download/v0.0.1/eng2reng_macos -o /tmp/eng2reng

# Run it.
chmod +x /tmp/eng2reng && /tmp/eng2reng Hello World

# Remove it.
rm /tmp/eng2reng
```

### Linux

```sh
# Fetch the file into /tmp/eng2reng.
curl -L https://github.com/davnotdev/renghlish/releases/download/v0.0.1/eng2reng_linux -o /tmp/eng2reng

# Run it.
chmod +x /tmp/eng2reng && /tmp/eng2reng Hello World

# Remove it.
rm /tmp/eng2reng
```

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
    word_buf.consonant(roots::root_c());
    word_buf.consonant(roots::root_k());
    word_buf
        .get_rows()
        .iter()
        .for_each(|row| eprintln!("{row}"));
}

```

## That's all

```plaintext
    │──╮╭──    ──╮    ──╮·  ·         ──╮  │
   ─┼──┼┼──    ──┼     ─┼──────     ────┼──┼
    │──╯╰──    ──╯      │·  ·╮      ╮ ──╯  │
  ╭─╯                 ──┤   ─┼─    ─┼─   ──┤
  │  ╲╱                 │    ├─     ├─     │
  ┴──                 ──┴    ╰╴     ╰╴     │
```
