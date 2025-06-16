# Compiling required C lib
`gcc -c adder/adder.c -o adder/adder.o`
`ar -rc ./adder/libadder.a ./adder/adder.o`

# Compiling Rust linking against C lib
`RUSTFLAGS='-L ./adder' cargo build`
