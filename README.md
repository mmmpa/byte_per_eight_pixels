[![CircleCI](https://circleci.com/gh/mmmpa/eight_px_uint_eight.svg?style=shield)](https://circleci.com/gh/mmmpa/eight_px_uint_eight)

# eight_px_uint_eight

This is a data structure for modules that display mono images receive data that have 8 pixels per byte. (ex: E-Paper)

This is `no_std` basically.

# HorizontalEightPxUintEight

For example, this make a 16 * 3 image 6 bytes.

(Lower x populates higher bit. `0123_4567`)

```
┌─ 1B ─┐
******** ********
******** ********
******** ********
└─ 8px─┘
```


# VerticalEightPxUintEight

For example, this make a 3 * 16 image 6 bytes.

(Lower y populates lower bit. `7654_3210`)

```
 ┌  *  *  *  ┐
 │  *  *  *  │
 │  *  *  *  │
1B  *  *  *  8px
 │  *  *  *  │
 │  *  *  *  │
 │  *  *  *  │
 └  *  *  *  ┘

    *  *  *
    *  *  *
    *  *  *
    *  *  *
    *  *  *
    *  *  *
    *  *  *
    *  *  *
```
