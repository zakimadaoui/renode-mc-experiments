# Getting started with Rust embedded and the blue pill board

This is a minimal example project for running Rust code on
a [blue pill][blue-pill] board. While the documentation in the embedded
Discovery book and the ARM/STM32 support crates is excellent, you need to
compile a lot of tiny bits from these sources to get a working setup for a
specific board. This is such a setup for the blue pill board.

## Install dependencies

* Install system packages *gdb-multiarch* and *openocd* (in Ubuntu; otherwise
  check the detailed [setup instructions][embedded-setup])
* Use nightly Rust from 2018-08-28 or newer (`rustup default nightly`)
* Add the compilation target (`rustup target add thumbv7m-none-eabi`)

## Connect the board

You need an ST-Link debugger/programmer. Connect the ST-Link header (aka SWD,
opposite side of USB) to the programmer and supply power via USB.

Now run

    openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg

You should end up with an output line like

    Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints

If not, check if your [installation is correct][embedded-verify].

## Run the example

The command

    cargo run

Will build the code, connect to the openocd session, flash it and wait. Execute
the code with the `continue` command (or just `c`).

## Next steps

* Learn how to step through your code in [gdb][embedded-debug].
* Check [Cargo config](./.cargo/config) and the [gdb script](./openocd.gdb) to
  see how Cargo is configured to cross-compile and connect with openocd.
* Browse the [examples][stm32-examples] in the HAL crate.

## Sources

This example is compiled from

* The [Cortex-M quickstart][cortex-m-quickstart] template.
* The [STM32F1 HAL][stm32-docs] docs.
* The [Rust embedded Discovery][embedded] book.

[blue-pill]: http://wiki.stm32duino.com/index.php?title=Blue_Pill
[cortex-m-quickstart]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart
[embedded]: https://docs.rust-embedded.org/discovery
[embedded-debug]: https://docs.rust-embedded.org/discovery/05-led-roulette/debug-it.html
[embedded-setup]: https://docs.rust-embedded.org/discovery/03-setup/index.html
[embedded-verify]: https://docs.rust-embedded.org/discovery/03-setup/verify.html
[stm32-docs]: https://docs.rs/stm32f1xx-hal/0.2.1/stm32f1xx_hal/
[stm32-examples]: https://docs.rs/crate/stm32f1xx-hal/0.2.1/source/examples/
