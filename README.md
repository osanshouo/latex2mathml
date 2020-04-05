# latex2mathml

`latex2mathml` provides a functionality to convert LaTeX math equations to MathML.
This crate is implemented in pure Rust, so it works in any environments if Rust works (including WebAssembly).

# Supported LaTeX commands

- Numbers, e.g. `0`, `3.14`, ...
- ASCII and Greek (and more) letters, e.g. `x`, `\alpha`, `\pi`, `\aleph`, ...
- Symbols, e.g., `\infty`, `\dagger`, `\angle`, `\Box`, `\partial`, ...
- Binary relations, e.g. `=`, `>`, `<`, ...
- Binary operations, e.g. `+`. `-`, `*`, `/`, `\times`, ...
- Basic LaTeX commands, e.g. `\sqrt`, `\frac`, `\sin`, ...
- Integrals, e.g., `\int`, `\int_0^\infty`, `\iint`, `\oint`, ...
- Big operators, e.g., `\sum`, `\prod`, `\bigcup_{i = 0}^\infty`, ...
- Font styles, e.g. `\mathrm`, `\mathbf`, `\bm`, `\mathit`, `\mathsf`, `\mathscr`, `\mathbb`, `\mathfrak`, `\texttt`.
  - MathML lacks calligraphic mathvariant: https://github.com/mathml-refresh/mathml/issues/61
- White spaces, e.g., `\!`, `\,`, `\:`, `\;`, `\ `, `\quad`, `\qquad`.
- Matrix, e.g. `\begin{matrix}`, `\begin{pmatrix}`, `\begin{bmatrix}`, `\begin{vmatrix}`.

See `examples/equations.rs` for examples.

## Unsupported LaTeX commands

- New line `\\`, except for ones in a matrix.
- Alignment `&`.
- Multi-line expressions, e.g. `\begin{align}..\end{align}`.
- Complicated sub/superscripts (`<mmultiscripts>`).

If a feature you need is lacked, feel free to open an issue.


# Usage

For a single LaTeX equation:

```rust
use latex2mathml::{latex_to_mathml, DisplayStyle};

let latex = r#"\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();
println!("{}", mathml);
```

For a document that includes LaTeX equations:

```rust
let text = r#"
Let us consider a rigid sphere (i.e., one having a spherical figure when tested in the stationary system) of radius $R$ 
which is at rest relative to the system ($K$), and whose centre coincides with the origin of $K$ then the equation of the 
surface of this sphere, which is moving with a velocity $v$ relative to $K$, is
$$\xi^2 + \eta^2 + \zeta^2 = R^2$$
"#;
let mathml = latex2mathml::replace(text).unwrap();
println!("{}", mathml);
```

See also `examples/equations.rs` and `examples/document.rs`.
