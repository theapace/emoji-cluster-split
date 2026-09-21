# emoji-cluster-split

Most emoji are not one codepoint. A skin tone is a separate modifier
codepoint tacked onto a base. A flag is two "regional indicator" letters.
A family is four people glued together with zero-width joiners. A keycap
digit is three codepoints wearing a trenchcoat. Code that does
`s.chars().count()` or slices a string by codepoint to "truncate to 10
characters" will happily cut a family emoji in half or split a flag into two
garbage indicator letters.

This tool takes text and prints it back split into the clusters a person
actually perceives as one emoji, along with the raw codepoints that make up
each one. It's meant for debugging exactly this class of bug: a truncation
routine that mangles emoji, a diff that shows a flag turning into two boxes,
a "character count" that disagrees with what's on screen.

## Usage

```
$ emoji-cluster-split "👨‍👩‍👧‍👦🇺🇸👋🏽"
  1  👨‍👩‍👧‍👦   U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F466
  2  🇺🇸        U+1F1FA U+1F1F8
  3  👋🏽        U+1F44B U+1F3FD

3 clusters
```

It also reads from stdin if you don't pass an argument:

```
$ echo "1️⃣2️⃣3️⃣" | emoji-cluster-split
  1  1️⃣   U+0031 U+FE0F U+20E3
  2  2️⃣   U+0032 U+FE0F U+20E3
  3  3️⃣   U+0033 U+FE0F U+20E3

3 clusters
```

Plain text passes through as one cluster per character, so you can run it on
mixed strings without special-casing anything:

```
$ emoji-cluster-split "hi 🤦🏽‍♀️!"
  1  h   U+0068
  2  i   U+0069
  3     U+0020
  4  🤦🏽‍♀️   U+1F926 U+1F3FD U+200D U+2640 U+FE0F
  5  !   U+0021

5 clusters
```

## What it knows about

- ZWJ sequences (families, couples, professions with a gender sign)
- Skin tone modifiers
- Flags (regional indicator pairs)
- Subdivision flags (tag base + tag letters + cancel tag, e.g. England/Scotland/Wales)
- Keycap sequences (digit or `#`/`*` + variation selector + combining keycap)
- Emoji vs. text presentation selectors

## What it doesn't do

This is not a general Unicode grapheme cluster (UAX #29) implementation. It
only groups the codepoints listed above; scripts with their own combining
rules (Hangul jamo, Indic conjuncts, and so on) are not handled and are not
in scope. If you need correct grapheme breaking for arbitrary text, use a
proper UAX #29 implementation instead.

## Building

Standard library only, no dependencies.

```
cargo build --release
```

## License

MIT, see LICENSE.
