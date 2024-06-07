cargo build --target thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf --example hello
cargo run --target thumbv7em-none-eabihf --example hello
# openocd -f interface/stlink-v2.cfg -f target/stm32h7x.cfg
# gdb-multiarch -q target/thumbv7em-none-eabihf/debug/examples/hello
# monitor arm semihosting enable