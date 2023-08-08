# Alabasta

Alabasta is an interactive Lambda Calculus Read-Eval-Print Loop (REPL) that supports arithmetic expressions, let expressions, and single-variable abstractions. 

## Features

* Enter Lambda Calculus expressions using a user-friendly syntax
* Support for arithmetic operations: addition, subtraction, multiplication, division, and modulus
* Let expressions for defining local variables within a scope
* Real-time evaluation and reduction of expressions using beta-reduction
* (Some) error handling for invalid expressions

## Examples

Lambda Abstraction:
```
λ-expr >> (\x.x)
=> (λ@x0. @x0)
```

Application:
```
λ-expr >> (\x.x) (\y.y)
=> (λ@x1. @x1)
```

Arithmetic:
```
λ-expr >> 2 + 3
=> 5
```

```
λ-expr >> let x = 5 in x + 2
=> 7
```

Let Expressions:
```
λ-expr >> let x = \y.y in x (\z.z)
=> (λ@x1. @x1)
```
```
λ-expr >> let a = 2 in let b = 3 in a * b + 1
=> 7
```

## Disclaimer

Alabasta is a Rust project undertaken as a learning experience. Please note that it's a work-in-progress, and there's limited error handling and reporting. As a result, there may be bugs and less-than-perfect code. 🚀✨
