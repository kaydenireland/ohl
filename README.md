# Ohl Language & Interpreter

A small programming language.

## Building

```bash
cd /path/to/project
cargo build --release
cargo install --path .
```

## Running

Use the oo command line interface to run commands:


### Print

Prints the contents of a file.
Use --numbered (-n) to number the lines in the print

```bash
oo print "file/path" --numbered
```

### Tokenize

Prints the tokens of a file.

```bash
oo tokenize "file/path"
```

### Parse

Prints the MTree of parsed tokens.
Use --debug (-d) to see full parse log.

```bash
oo parse "file/path" --debug
```

## Alternative

If running through cargo project itself, replace "oo" with "cargo run".

# Language Overview

The language supports:
- Functions with parameters and return types
- Integer, float, char, string and boolean types
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`, `^/`
- Relational: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Assignment: `+=`, `-=`, `*=`, `/=`, `^=`, `^/=`
- Incremental: `++`, `--`, `**`
- Logical: `&&`/`and`, `||`/`or`, `^^`/`xor`, `!`/`not`
- Control flow: `if`-`else`, `match`, `default`, `for`, `loop`, `while`, `break`, `continue`, `repeat`, `return`
- Variable declarations: `let x: int = 5;`
- Assignments: `x = 10;`
- Function calls: `factorial(n)`
- Print statement: `print(result);`

## Example

```ohl
private int factorial(n: int) {
    if (n < 2) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

public null main() {
    let result: int = factorial(5);
    print result;
}
```

## Semantic Analysis Output

The compiler reports semantic errors with details:

```
✓ Semantic analysis completed with 3 error(s):
  1. Variable 'undefined_var' not declared
  2. Type mismatch for 'x': expected Int, found Bool
  3. Function 'unknown_func' expects 1 arg but 2 provided
```