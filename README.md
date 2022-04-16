# Update Functions
A small package for calling a function at a desired interval
# Usage
```
fn main() {
let dir = Duration::new(1, 0) // One second and 0 ms
UpdateFunctions::update(dir, Update)
}

pub fn Update() {
println!("Updating!");
}
```
# Installation
In `cargo.toml`
```
UpdateFunctions = "0.1.3"
```