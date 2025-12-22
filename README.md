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

### Size

Prints the size of a file in bytes.
```bash
oo size "file/path"
```

### Tokenize

Prints the tokens of a file.

```bash
oo tokenize "file/path"
```

### Parse

Prints the MTree of parsed tokens.
Use --debug (-d) to see full log.

```bash
oo parse "file/path" --debug
```

### Convert

Prints the converted semantic tree from the parse tree
Use --debug (-d) to see full log.

```bash
oo convert "file/path" --debug
```

### Analyze

Checks the input code for warnings and errors.
Use --debug (-d) to see full log.

```bash
oo analyze "file/path" --debug
```

### Run

Optimizes and runs the input file.
Use --debug (-d) to see full log.
Use --time (-t) to see length of execution
Use --warnings (-w) to hide warnings

```bash
oo run "file/path" --debug
```

## Alternative

If running through cargo project itself, replace "oo" with "cargo run".

# Language Overview

The language supports:
- Functions with parameters and return types
- Integer, float, char, string, and boolean types
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`, `^/`
- Relational: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Assignment: `+=`, `-=`, `*=`, `/=`, `^=`, `^/=`, `++`, `--`, `**` 
- Logical: `&&`/`and`, `||`/`or`, `^^`/`xor`, `!`/`not`
- Control flow: `if`-`else`, `match`, `default`, `for`, `loop`, `while`, `break`, `continue`, `repeat`, `return`
- Variable declarations: `let x: int = 5;`
- Assignments: `x = 10;`
- Function Scoping: `public`, `protected`, `public`
- Function calls: `factorial(n)`
- Print statement: `print(result);`
- Comments: `//`, `/* */`

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
    print(result);
}
```

## Semantic Analysis Output

The compiler reports semantic errors with details:

```
Analysis completed with 14 error(s):
  1. Function 'foo' already declared
  2. Function 'noReturn' declares return type INT but has no return statement
  3. Variable 'a' is already declared in this scope
  4. Variable 'a' is not declared
  5. Variable 'a' is not declared
  6. Assignment type mismatch for 'a': INT vs BOOLEAN
  7. Unary NOT requires Bool, found INT
  8. Invalid operands for ADD: INT and STRING
  9. Comparison requires numeric types, got INT and STRING
  10. Logical operator AND requires Bool operands
  11. While condition must be Bool, found INT
  12. Function 'foo' expects 0 args but 1 provided
  13. Call to unknown function 'bar'
  14. Postfix INCREMENT requires a numeric variable
```

## Execution Output

```bash
Running example.ohl

120

Completed execution in 0.0005s
```