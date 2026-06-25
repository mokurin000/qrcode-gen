## Tooling

Prefer `bash -c '...'` to run every commands, never use nushell, powershell or cmd.

`rg` is provided to quickly grep.

## Code style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — a set of recommendations on how to design and present APIs for Rust, authored by the Rust library team.

### Naming
- **C-CASE**: Casing conforms to RFC 430 (`snake_case`, `CamelCase`, `SCREAMING_SNAKE_CASE`)
- **C-CONV**: Ad-hoc conversions follow `as_`, `to_`, `into_` conventions
- **C-GETTER**: Getter names follow Rust convention (no `get_` prefix for direct field access)
- **C-ITER**: Collection iterators use `iter()`, `iter_mut()`, `into_iter()`
- **C-ITER-TY**: Iterator type names match the methods that produce them
- **C-FEATURE**: Feature names are free of placeholder words
- **C-WORD-ORDER**: Names use a consistent word order

### Interoperability
- **C-COMMON-TRAITS**: Types eagerly implement common traits: `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`
- **C-CONV-TRAITS**: Conversions use standard traits `From`, `AsRef`, `AsMut`
- **C-COLLECT**: Collections implement `FromIterator` and `Extend`
- **C-SERDE**: Data structures implement Serde's `Serialize`/`Deserialize` where appropriate
- **C-SEND-SYNC**: Types are `Send` and `Sync` where possible
- **C-GOOD-ERR**: Error types are meaningful and well-behaved (implement `std::error::Error`)
- **C-NUM-FMT**: Binary number types provide `Hex`, `Octal`, `Binary` formatting
- **C-RW-VALUE**: Generic reader/writer functions take `R: Read` and `W: Write` by value

### Macros
- **C-EVOCATIVE**: Input syntax is evocative of the output
- **C-MACRO-ATTR**: Macros compose well with attributes
- **C-ANYWHERE**: Item macros work anywhere items are allowed
- **C-MACRO-VIS**: Item macros support visibility specifiers
- **C-MACRO-TY**: Type fragments are flexible

### Documentation
- **C-CRATE-DOC**: Crate-level docs are thorough and include examples
- **C-EXAMPLE**: All public items have a rustdoc example
- **C-QUESTION-MARK**: Examples use `?`, not `try!`, not `unwrap`
- **C-FAILURE**: Function docs include error, panic, and safety considerations
- **C-LINK**: Prose contains hyperlinks to relevant things
- **C-METADATA**: `Cargo.toml` includes all common metadata (authors, description, license, homepage, documentation, repository, keywords, categories)
- **C-RELNOTES**: Release notes document all significant changes
- **C-HIDDEN**: Rustdoc does not show unhelpful implementation details

### Predictability
- **C-SMART-PTR**: Smart pointers do not add inherent methods
- **C-CONV-SPECIFIC**: Conversions live on the most specific type involved
- **C-METHOD**: Functions with a clear receiver are methods
- **C-NO-OUT**: Functions do not take out-parameters
- **C-OVERLOAD**: Operator overloads are unsurprising
- **C-DEREF**: Only smart pointers implement `Deref`/`DerefMut`
- **C-CTOR**: Constructors are static, inherent methods

### Flexibility
- **C-INTERMEDIATE**: Functions expose intermediate results to avoid duplicate work
- **C-CALLER-CONTROL**: Caller decides where to copy and place data
- **C-GENERIC**: Functions minimize assumptions about parameters by using generics
- **C-OBJECT**: Traits are object-safe if they may be useful as trait objects

### Type Safety
- **C-NEWTYPE**: Newtypes provide static distinctions
- **C-CUSTOM-TYPE**: Arguments convey meaning through types, not `bool` or `Option`
- **C-BITFLAG**: Types for a set of flags use `bitflags`, not enums
- **C-BUILDER**: Builders enable construction of complex values

### Dependability
- **C-VALIDATE**: Functions validate their arguments
- **C-DTOR-FAIL**: Destructors never fail
- **C-DTOR-BLOCK**: Destructors that may block have alternatives

### Debuggability
- **C-DEBUG**: All public types implement `Debug`
- **C-DEBUG-NONEMPTY**: `Debug` representation is never empty

### Future Proofing
- **C-SEALED**: Sealed traits protect against downstream implementations
- **C-STRUCT-PRIVATE**: Structs have private fields
- **C-NEWTYPE-HIDE**: Newtypes encapsulate implementation details
- **C-STRUCT-BOUNDS**: Data structures do not duplicate derived trait bounds

### Necessities
- **C-STABLE**: Public dependencies of a stable crate are stable
- **C-PERMISSIVE**: Crate and its dependencies have a permissive license
