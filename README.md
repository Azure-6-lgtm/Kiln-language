# Kiln

**A language for writing Readable, Reliable and Reasonable code.**

> *source in, solid out*

Kiln is a small, high-level programming language with a C-style feel, written in Rust. Source files (`.kiln`) are lexed, parsed into an AST, and run by a tree-walking interpreter.

**Status:** early alpha (`0.2.0 alpha pre-release`). The language is small and changing quickly.

## Example

```kiln
// Kiln 
func main() -> void {
    const can_you_vote = check_voting_age(36);

    when (can_you_vote) {
        print("You can vote");
    }
    otherwise {
        print("You cannot vote");
    }
}

func check_voting_age(n: int) -> bool {
    when (n >= 18) {
        return True;
    } otherwise {
        return False;
    }
}
```

Output:

```
You can vote
```

## Getting started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo

### Build

```sh
git clone https://github.com/Azure-6-lgtm/Kiln-language.git
cd Kiln-language/Kiln
cargo build --release
```

### Run

```sh
cargo run -- hello.kiln
```

Or run the built binary directly:

```sh
Kiln hello.kiln
Kiln --ver
```

### Using the installation script
```sh
chmod +x install-kiln.sh
./install-kiln.sh
```

### Using the unininstallation script
```sh
chmod +x uninstall-kiln.sh
./uninstall.sh
```

Usage: `Kiln [OPTION] [FILE]`

- Files must end in `.kiln`.
- `--ver` prints the version.
- Running with no arguments prints the usage line.
- Execution starts at `func main()`.

## Language guide

### Comments

```kiln
// single line

/* multi
   line (does not nest) */
```

### Variables

`const` is immutable. `mut` is mutable and can be reassigned.

```kiln
const name = "Kiln";
mut count = 0;
count = count + 1;
```

Assigning to a `const` is an error (E020).

### Types

| Type | Notes |
|---|---|
| `int` | whole numbers |
| `float` | decimal numbers |
| `string` | double-quoted text |
| `bool` | `True` or `False` |
| `void` | no value |

Numbers are stored as 64-bit floats internally. A function declared `-> int` must return a whole number.

### Operators

| Kind | Operators |
|---|---|
| Arithmetic | `+` `-` `*` `/` and unary `-` |
| Comparison | `==` `!=` `<` `<=` `>` `>=` |
| Strings | `+` (string + string), `==`, `!=` |

Use parentheses to group. Multiplication and division bind tighter than addition and subtraction, which bind tighter than comparisons.

### Conditionals

Kiln uses `when` and `otherwise`. Chain them with `otherwise when`.

```kiln
func grade(score: int) -> string {
    when (score >= 90 and score <= 100) {
        return "A";
    }
    otherwise when (score >= 80) {
        return "B";
    }
    otherwise {
        return "C";
    }
}
```

The condition must be a `bool` (E019).
Instead of symbols, the keywords `and`, `or` and `not` are used.

### Functions

Parameters are typed. The return type is optional: leave off `-> type` and the function still works.

```kiln
func some() {
    return 32;
}
```

When a return type is declared, Kiln checks it when the function returns (E013). A function declared `-> void` cannot return a value (E002).

Functions can call themselves. Recursive functions also work but are not recommended as loops are here.:

```kiln
func factorial(n: int) -> int {
    when (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

func main() {
    print("5! =", factorial(5));
}
```
### Loops 
`while` loop is currently available in 0.2.0 . `for` and `do while` loops are still W.I.P and are expected by 0.2.2

``` Kiln
func factorial(n: int) -> int {
    mut result int = 1;
    mut i = n;

    while (i > 1) {
        result = result * i;
        i = i - 1; // increment operators are still W.I.P
    }

    return result;
}

func main() {
    print("5! =", factorial(5));
}
```

### Printing

`print` is built in. It takes any number of arguments, separates them with spaces, and ends with a newline.
However, it will be removed once the standard library is implemented 

```kiln
print("Hello,", "Kiln");   // Hello, Kiln
```

### Reserved words

`func` `const` `mut` `return` `when` `otherwise` `True` `False` `int` `float` `string` `bool` `void` `while`

Reserved for future features (not implemented yet): `require` `struct` `enum` `switch` `case` `for` `in` `try` ` do`

## Errors

Errors stop the program and print a message with a code.

```
Kiln: lexer error at 3:5: Unexpected character '@' [E023]
Kiln: parser error at 4:12: Expected Semicolon, found RightBrace [E001]
Kiln: runtime error: Undefined variable: x [E015]
```

Lexer and parser errors include a line and column. Runtime errors do not yet.

| Code | Stage | Meaning |
|---|---|---|
| E001 | Parser | Expected one token, found another |
| E002 | Parser | A `void` function cannot return a value |
| E003 | Parser | Expected a function at the top level |
| E004 | Parser | Expected a variable name |
| E005 | Parser | Expected a function name |
| E006 | Parser | Expected a parameter name |
| E007 | Parser | Expected a parameter type |
| E008 | Parser | Expected a return type |
| E009 | Parser | Unexpected token in an expression |
| E010 | Lexer | Multi-line comment not closed |
| E011 | Runtime | Undefined function |
| E012 | Runtime | Wrong number of arguments |
| E013 | Runtime | Returned type does not match the declared return type |
| E014 | Runtime | Invalid number |
| E015 | Runtime | Undefined variable |
| E016 | Runtime | Division by zero |
| E017 | Runtime | Invalid operand types for an operation |
| E018 | Runtime | Only named functions can be called |
| E019 | Runtime | Condition is not a `bool` |
| E020 | Runtime | Cannot assign to an immutable variable |
| E021 | Runtime | Cannot assign to an undefined variable |
| E022 | Lexer | Unterminated string |
| E023 | Lexer | Unexpected character |
| E024 | Runtime | Not requires a boolean condition |
| E025 | Runtime | And requires a boolean condition
| E026 | Runtime | Or requires a boolean condition
| E027 | Runtime | While requires a boolean condition

## Current limitations

- No `for` or `do while` loops. Use `while` and recursion for now`
- No `require`, `struct`, `enum`, `switch` or `try` yet (reserved keywords).
- No `%` or compound assignment such as `+=`.
- Parameter types must be written but are not checked when a function is called. Mismatched operands still fail (E017).
- Booleans cannot be compared with `==` or `!=`.
- Strings have no escape sequences such as `\n`.
- No block scoping: variables declared inside a `when` block stay visible for the rest of the function.

## Project layout

```
Kiln/
└── src/
    ├── main.rs         command-line entry point
    ├── token.rs        token definitions
    ├── lexer.rs        source text -> tokens
    ├── parser.rs       tokens -> AST
    ├── ast.rs          AST node definitions
    └── interpreter.rs  runs the AST
```

## Contributing

Issues and pull requests are welcome. <br>
For more information, See [Contributing](contributing.md)

## License

The entirety of Kiln-language is licensed under Mozilla Public License 2.0
