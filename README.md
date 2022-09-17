# Init with Rust lenguaje

## install 

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### install cargo-edit (package manager)

    cargo install cargo-edit

### create proyect 

    cargo new name-proyect

### crete library

    cargo new --lib name-library

## variables
```Rust
let name_vari_no_mut = "hello, world";
let mut name_vari_mut = "hello, world";
```
### variable with reference to type
```Rust
// static in stack
let name_vari: &str = "hello, world";
```
### create variable with the type construtor

```Rust 
// dinamic in heap
let name_vari: String = String::from("content vari");
```

### reassing variable
```Rust
let x: String = String::from("content vari");
y = x; // in this line "y" get the content in memory and "x" loses this content
m = &x; // in this line "m" contain a teference to x

let w: &str = "Hello";
let z = w; // in this line "z" get value off "w", but not points to the same memory position
```
## functions

```Rust
fn name_function(){
    // logic funciton
}```




