# STM32F103RB

This projects was made with cortex-m-quickstart

# What is this?

I want to try to create a library to control my stm32 board in a rusty way. Currently it is able to access gpio and change the state of the on board LED, so that should give you a clue on how far this project currently is.

The next goal is to write a Timer abstraction for a delay function.

# What do you need if you want to try this project?

A stm32f103rb nucleo board and rust beta/nightly.

# How to run?

One terminal:

openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg
Other terminal:

cargo run
This will open a gdb session to debug your project.

# License

This template is licensed under either of

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
