# sesshomaru
A small crate for creating a window and handling basic keyboard input using rust-sdl2.
## Warning.
This is not thread safe, It uses unsafe almost everywhere and it contains globals.
## Example
```
use sesshomaru;
use sesshomaru::Scancode;
fn main() {
    sesshomaru::init("Test Window", 640, 480);
    loop {
        // Returns false if the program quit was requested, true otherwise.
        let result = sesshomaru::poll();
        if !result || sesshomaru::key_pressed(Scancode::Escape) {
            return;
        }
    }
}
```