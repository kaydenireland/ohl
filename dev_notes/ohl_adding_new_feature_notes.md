# Adding a New Feature to Ohl

A language feature must travel through the compiler pipeline:

```text
Ohl source
   ↓
Lexer
   ↓
Parser / MTree
   ↓
STree
   ↓
Analyzer
   ↓
Intermediate IR
   ↓
Machine IR
   ↓
Stack allocation
   ↓
Legalization
   ↓
x86-64 Assembly
   ↓
Assembler / Executable
```

The safest approach is to work **from the language downward**, one abstraction level at a time.

---

## 1. Define the Feature

Before changing code, decide:

- Syntax
- Semantics
- Types involved
- Whether it produces a value
- Precedence / associativity, if applicable
- Valid and invalid uses
- Edge cases

Example:

```ohl
int x = 10 % 3;
```

For `%`, define that it is a binary integer operation producing an integer result.

---

## 2. Update the Lexer

Add a token if the syntax needs one.

For example:

```rust
pub enum Token {
    ...
    MODULO,
    ...
}
```

Then teach the lexer to recognize `%`.

### Why?

The lexer answers:

> What pieces of syntax did the programmer write?

It should not decide what those pieces mean.

### Checkpoint

Verify:

```text
10 % 3
```

produces the expected token sequence.

---

## 3. Update the Parser / MTree

Teach the parser where the new syntax is valid.

For operators, this commonly means updating Pratt-parser precedence and/or operator parsing.

Test examples such as:

```ohl
10 % 3
10 + 5 % 3
(10 + 5) % 3
```

### Why?

The parser answers:

> How are these tokens structured?

The resulting MTree should have the correct tree structure and precedence.

---

## 4. Update the STree

Determine whether the STree needs a new node or whether an existing node can represent the feature.

For a binary operator, an existing structure such as:

```rust
STree::EXPR {
    left,
    operator,
    right,
}
```

may already be sufficient.

Add the operator to the relevant enum:

```rust
pub enum Operator {
    ...
    MODULO,
}
```

### Why?

This is where syntax becomes a meaningful language construct.

---

## 5. Update Semantic Analysis

Determine whether the analyzer needs changes.

Ask:

- Are the operands valid?
- Are the types compatible?
- Does the feature introduce new scope?
- Does it affect control flow?
- Are there invalid combinations to reject?

For example, if `%` only accepts integers, the analyzer should reject invalid operand types.

### Why?

The analyzer answers:

> Is this program valid?

The backend should not have to perform basic language-level validation.

---

## 6. Add the Feature to the Intermediate IR

Add the operation to the Intermediate representation.

For example:

```rust
pub enum IntermediateBinaryOperator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
}
```

Then update Intermediate lowering.

Conceptually:

```text
STree expression
        ↓
Intermediate instruction
```

For example:

```text
tmp.0 = tmp.1 % tmp.2
```

### Why?

The Intermediate IR describes **what the program wants to do**, without being tied to x86.

The same Intermediate IR should eventually be usable by another backend such as ARM64 or RISC-V.

### Checkpoint

Dump the Intermediate IR and verify that the new feature appears correctly.

---

## 7. Update Machine IR

Now determine how the Intermediate operation maps to actual machine operations.

Simple operations may become a normal machine binary instruction:

```text
ADD
SUBTRACT
MULTIPLY
```

More complicated x86 operations may require several instructions.

For example, signed division requires:

```text
MOVE src1 → AX
CDQ
IDIV src2
MOVE AX → dst
```

while remainder requires:

```text
MOVE src1 → AX
CDQ
IDIV src2
MOVE DX → dst
```

### Why?

Machine IR is much closer to the target architecture than Intermediate IR.

---

## 8. Update Stack Allocation

This step is easy to forget.

Every MachineInstruction that can contain:

```rust
Operand::PSEUDO(...)
```

must be handled by stack allocation.

For example:

```text
tmp.0
```

eventually becomes something like:

```text
-4(%rbp)
```

Make sure new instructions are included.

Current examples include:

```text
MOVE
UNARY
BINARY
IDIV
```

### Important rule

Whenever you add a new MachineInstruction, immediately ask:

> Can this instruction contain a PSEUDO operand?

If yes, update stack allocation.

---

## 9. Update Legalization

Now ask:

> Is this Machine IR instruction actually legal on x86-64?

Legalization handles restrictions of the target architecture.

### Memory-to-memory MOVE

Invalid:

```asm
movl -4(%rbp), -8(%rbp)
```

Legalized:

```asm
movl -4(%rbp), %r10d
movl %r10d, -8(%rbp)
```

### `imul` with a memory destination

For the form used by the compiler, an instruction such as:

```asm
imull $3, -4(%rbp)
```

must be transformed so the destination is a register:

```asm
movl -4(%rbp), %r11d
imull $3, %r11d
movl %r11d, -4(%rbp)
```

### `idiv` with an immediate

Invalid:

```asm
idivl $3
```

Legalized:

```asm
movl $3, %r10d
idivl %r10d
```

### Why?

Legalization converts:

```text
machine operations that express the right computation
```

into:

```text
machine operations that can actually be encoded by x86
```

---

## 10. Update x86-64 Code Generation

Add the required instruction/operator to the x86 backend.

At this point, code generation should mostly translate Machine IR into assembly.

Remember that your backend uses AT&T syntax:

```text
source, destination
```

For example:

```asm
addl %r10d, -4(%rbp)
```

means:

```text
-4(%rbp) = -4(%rbp) + %r10d
```

### Goal

The code generator should not be making major language decisions. Those decisions should already have happened in the earlier stages.

---

## 11. Assemble and Execute

Start with a tiny test.

For example:

```ohl
return 10 % 3;
```

Then test progressively more complicated cases:

```ohl
return 10 % 3 + 2;
```

```ohl
int x = 10;
int y = 3;
return x % y;
```

```ohl
return (10 + 5) % 3;
```

Also test negative values and other relevant edge cases.

Check both:

1. Does the generated assembly assemble?
2. Does the executable produce the correct result?

Valid assembly does **not** necessarily mean correct code.

---

## 12. Add Regression Tests

Once the feature works, keep tests for it.

At minimum test:

```text
basic case
variables
nested expressions
edge cases
invalid cases
```

For arithmetic, also test:

```text
positive values
negative values
zero
large values
```

as appropriate.

### Why?

Compiler changes can easily break older features in:

- lowering
- stack allocation
- legalization
- code generation

Regression tests catch this.

---

## 13. Update Documentation

Document the feature after implementation.

Include:

- Syntax
- Meaning
- Types
- Examples
- Restrictions
- Edge cases

The language documentation should eventually agree with the compiler implementation.

---

# Quick Checklist

When adding a feature:

```text
[ ] 1. Define syntax and semantics
[ ] 2. Lexer
[ ] 3. Parser / MTree
[ ] 4. STree
[ ] 5. Semantic analysis
[ ] 6. Intermediate IR
[ ] 7. Machine IR
[ ] 8. Stack allocation
[ ] 9. Legalization
[ ] 10. x86-64 code generation
[ ] 11. Assemble and execute
[ ] 12. Regression tests
[ ] 13. Documentation
```

---

# The Mental Model

When you get lost, remember what each stage is responsible for:

| Stage | Main question |
|---|---|
| Lexer | What tokens did the programmer write? |
| Parser / MTree | How are those tokens structured? |
| STree | What does that structure mean? |
| Analyzer | Is it valid? |
| Intermediate IR | What operations does the program perform? |
| Machine IR | How can the target machine perform those operations? |
| Stack allocation | Where do the pseudo-values live? |
| Legalization | Are these machine operations legal for x86? |
| Codegen | What exact assembly represents them? |
| Assembler | Can the assembly become machine code? |
| Execution | Does the resulting program behave correctly? |

## Debugging Rule

If something goes wrong, find the **first stage where the representation becomes incorrect**.

```text
Source / tokens wrong
    → lexer or parser

MTree wrong
    → parser

STree wrong
    → MTree → STree conversion

Intermediate IR wrong
    → Intermediate lowering

Machine IR wrong
    → Machine lowering

Pseudo operands remain
    → stack allocation

Machine instruction is illegal
    → legalization

Assembly text is wrong
    → x86-64 codegen
```

Do not immediately change the code generator just because the final assembly is wrong. Trace the value backward until you find where it first became wrong.

> **Move downward one abstraction level at a time.**
