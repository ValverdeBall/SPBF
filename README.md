# SPBF

**SPBF** (СП-BeginnerFriendly) is the beginner tier of the **СП** (SP) language family: a light syntax that mixes Lua and Python, meant to be easy to read and hard to get wrong. It is written in Rust and is a work in progress.

Source files use the `.spbf` extension. The entry point of a project is `main.spbf`.

## A taste

```
// comments start with //
fn divmod(a, b)
    return a \ b, a % b

priv (q, r) = divmod(17, 5)
print(q, r)

for i = 1, 3
    print(i)

priv scores = {anna = 10, bob = 7}
print(scores["anna"] ?? 0)

try
    priv x = 1 / 0
catch e
    print(e.message)
```

## Language at a glance

- Python-style indentation blocks (spaces only), no semicolons, no colons after `if` / `while`
- Variables are declared with `priv` and are block-scoped; no implicit globals
- No `null` at all: a missing key or index is a runtime error, unless handled with `??`
- Strict conditions: only real booleans, no truthiness
- Separate `int` (arbitrary precision) and `float` types; `/` is regular division, `\` is integer division
- Arrays `[1, 2, 3]` and dicts `{key = value}`, zero-based, negative indexes allowed
- Functions with default parameters and multiple return values
- `try` / `catch` / `throw`
- Unicode identifiers (Cyrillic works)
- `import` is for built-in modules; other project files see each other automatically, and names are qualified by file name (`utils.helper()`)

## Status

Early development. The language design is settled; the implementation is being built bottom-up.

- [x] Token and diagnostic types
- [ ] Lexer (numbers, operators, identifiers, and columns done; strings, newlines and indentation next)
- [ ] Parser and AST
- [ ] Register-based bytecode compiler and interpreter
- [ ] `.sp` bytecode files
- [ ] JIT

## Building

```
cargo run
```

## The СП family

SPBF is the first tier to be implemented. The rest of the family is planned:

| Tier | Idea |
| --- | --- |
| Low Level СП | C-like syntax, full control, compiled to native code through LLVM |
| СП-Intermediate | Memory control with compiler checks, lighter syntax, cross-platform |
| СП-Medium | Kotlin-like syntax, garbage collected, runs on its own JIT |
| СП-BeginnerFriendly | This project |
