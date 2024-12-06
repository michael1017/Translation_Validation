## Changes
Remove main function in C

## Commands

## Note
Different function interface in C and Rust
=> These two original functions are obviously non-equal

Failed to solve the issue of the follow message, since `case` is a key work. Besides, ...35-union would have syntax error.

```
reason for conflict at #this: type classes differ

signed int
struct _284712733721269481886865427472035631735 {
  unsigned int case;
  union _284712733721269481886865427472035631735-union cases;
}

```

I tried to change the return value of None to -1 in Rust. By doing so, I can set return type as int32 rather than Option<??>. 

However, the original C function would not set return value to -1, it just do nothing. So after my change, the programs are still non-equal.