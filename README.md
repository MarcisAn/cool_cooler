# Cool Cooler

A Work In Progress library for reading/writing Cool data files.

Current functionality includes listing datasets present in a file.

Requires HDF5 library. Installable on linux or WSL with `sudo apt-get install libhdf5-serial-dev`

## Example:

```rust
use CoolCooler::*;

fn main() {
    let cooler = CoolCooler::new("coolfile.cool");
    let datasets_in_file = cooler.get_datasets();
    println!("{:?}", datasets_in_file);
}
```
