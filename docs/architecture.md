# Project Architecture
Armadillo performs two main functions: parsing a program and running a parsed
program. Parsing a program transforms source text
(a string containing ECMAScript) into an abstract syntax tree, or AST.
That AST is then evaluated in order to execute the program.

## File Structure
Armadillo's `src` directory has two subdirectories:
`parsing` and `runtime`.

`parsing`: Transforms source text into an AST. Contains a tokenizer and an AST
generator.

`runtime`: Executes an AST.
  - `interface`: Provides an interface with one function: executing an AST.
  - `internals`: Contains necessary internal types and functions for executing
    an AST. These types should not be used by clients of the `runtime` module.
    - `types`: Contains representations of ECMAScript language and
      specification types that are used internally.
    - `methods`: Contains method implementations for language and specification
      types.
    - `builtins`: Contains objects and values that are ECMAScript built-ins.
