# deref-rs

A simple Rust autodereference implementation that provides macros and derive macros to easily implement the `Deref` and `DerefMut` traits for your custom types.

## Features

- **Macro-based implementation**: Simple procedural macros for quick `Deref` and `DerefMut` implementation
- **Derive macro support**: Attribute-based derive macros for more ergonomic usage
- **Generic type support**: Works with both regular types and generic types
- **Field access**: Supports both named fields and tuple-style field access
- **Automatic Deref implementation**: When using `deref_mut!` or `DerefMut`, `Deref` is automatically implemented

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deref = "0.1.0"
```

## Usage

### Using Procedural Macros

#### Basic Usage

```rust
use deref::{deref, deref_mut};

struct MyType {
    field: TargetType,
}

struct TargetType;

// Implement Deref for MyType
deref!(MyType, TargetType, field);

// Implement both Deref and DerefMut for MyType
// Note: This automatically implements Deref as well
deref_mut!(MyType, TargetType, field);
```

#### With Generic Types

```rust
use deref::{deref, deref_mut};
struct SrVec<T> {
    vec: Vec<T>,
}

// Implement Deref for a generic type
deref!(<T>, SrVec<T>, Vec<T>, vec);

// Implement both Deref and DerefMut for a generic type
// Note: This automatically implements Deref as well
deref_mut!(<T>, SrVec<T>, Vec<T>, vec);
```

### Using Derive Macros

#### Basic Usage

```rust
use deref::{Deref, DerefMut};
#[derive(Deref)]
struct Hello<T> {
    #[auto_ref]
    inner: T,
}

#[derive(DerefMut)]
struct HelloMut<T> {
    #[auto_ref]
    inner: T,
}
// Note: DerefMut automatically implements Deref as well
```

#### With Named Fields

```rust
use deref::Deref;
#[derive(Deref)]
struct Wrapper {
    #[auto_ref]
    data: String,
}

fn main() {
    let wrapper = Wrapper {
        data: "Hello, World!".to_string(),
    };
    
    // Can access String methods directly through wrapper
    println!("Length: {}", wrapper.len());
}
```

#### With Tuple Structs

```rust
use deref_rs::Deref;

#[derive(Deref)]
struct TupleWrapper(String);

#[derive(Deref)]
struct GenericTupleWrapper<T>(#[auto_ref] T);

fn main() {
    let wrapper = TupleWrapper("Hello, Tuple!".to_string());
    
    // Can access String methods directly through wrapper
    println!("Length: {}", wrapper.len());
}
```

## Examples

### Smart Pointer Implementation

```rust
use deref_rs::Deref;
use std::ops::Deref as DerefTrait;

#[derive(Deref)]
struct MyBox<T> {
    #[auto_ref]
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox { value }
    }
}

fn main() {
    let my_box = MyBox::new(42);
    
    // Can access the inner value directly
    println!("Value: {}", *my_box);
    
    // Can call methods on the inner type
    let string_box = MyBox::new("Hello".to_string());
    println!("Length: {}", string_box.len());
}
```

### Nested Dereferencing

```rust
use deref::{Deref, DerefMut, deref_mut};
#[derive(Debug, DerefMut)]
struct Inner<T> {
    #[auto_ref]
    value: T,
}
// Note: DerefMut automatically implements Deref for Inner<T>

#[derive(Debug)]
struct Outer<T> {
    inner: Inner<T>,
}

// Implement DerefMut for Outer using the macro
// Note: This automatically implements Deref as well
deref_mut!(<T>, Outer<T>, T, inner);

fn main() {
    let mut outer = Outer {
        inner: Inner {
            value: "Hello, World".to_string(),
        },
    };

    println!("{:?}", outer);

    // Can modify the inner value directly
    *outer += "!";
    
    println!("{:?}", outer);
}
```

## API Reference

### Macros

#### `deref!` Macro

Implements the `Deref` trait for a struct.

```rust
deref!(TYPE, TARGET_TYPE, FIELD)
deref!(GENERIC_PARAMS; TYPE, TARGET_TYPE, FIELD)
```

#### `deref_mut!` Macro

Implements both `Deref` and `DerefMut` traits for a struct.

```rust
deref_mut!(TYPE, TARGET_TYPE, FIELD)
deref_mut!(GENERIC_PARAMS, TYPE, TARGET_TYPE, FIELD)
```

**Note**: The `deref_mut!` macro automatically implements both `Deref` and `DerefMut` traits. You don't need to separately use `deref!` when using `deref_mut!`.

### Derive Macros

#### `Deref` Derive Macro

Implements the `Deref` trait using the `#[auto_ref]` attribute to mark the target field.

#### `DerefMut` Derive Macro

Implements both `Deref` and `DerefMut` traits using the `#[auto_ref]` attribute to mark the target field.

**Note**: The `DerefMut` derive macro automatically implements both `Deref` and `DerefMut` traits. You don't need to separately derive `Deref` when deriving `DerefMut`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0
- Initial release
- Basic `deref!` and `deref_mut!` macros
- `Deref` and `DerefMut` derive macros
- Support for generic types
- Automatic Deref implementation when using DerefMut
