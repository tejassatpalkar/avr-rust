I went down a rabbit hole trying to be able to compile rust and flash my arduino uno knockoff (Redboard). 

Assuming you have all the avr toolchians, rust nightly, etc, 

```bash
cargo build --release 
```

Which creates 

```bash
[ts@crapbook avr-rust]$ file target/avr-none/release/avr-rust.elf 
target/avr-none/release/avr-rust.elf: cannot open `target/avr-none/release/avr-rust.elf' (No such file or directory)
```

## Flashing the board
```bash
[ts@crapbook avr-rust]$ file target/avr-none/release/avr-rust.elf 
target/avr-none/release/avr-rust.elf: ELF 32-bit LSB executable, Atmel AVR 8-bit, version 1 (SYSV), statically linked, not stripped
[ts@crapbook avr-rust]$ avrdude -patmega328p -carduino -P /dev/ttyUSB0  -b115200 -D -Uflash:w:target/avr-none/release/avr-rust.elf:e
Reading 112 bytes for flash from input file avr-rust.elf
Writing 112 bytes to flash
Writing | ################################################## | 100% 0.09 s 
Reading | ################################################## | 100% 0.04 s 
112 bytes of flash verified

Avrdude done.  Thank you.
```


TODO:
* Actually execute code
* Do I need a memory.hex file ?
* Test out some of the avr-rust crates
* Make a Fun Project out of this :)

Sources: 
https://doc.rust-lang.org/nightly/rustc/platform-support/avr-none.html
https://book.avr-rust.org/004-flashing-a-crate-to-chip.html



