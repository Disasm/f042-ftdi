# `f042-ftdi`

> A proof-of-concept FT2232D emulation firmware for an STM32F042 microcontroller.

## Pinout

| FTDI signal  | STM32 pin |
|--------------|-----------|
| ADBUS0 (TCK) | PA5       |
| ADBUS1 (TDI) | PA7       |
| ADBUS2 (TDO) | PA6       |
| ADBUS3 (TMS) | PA4       |
|              |           |
| BDBUS0 (TXD) | PA2       |
| BDBUS1 (RXD) | PA3       |

## Build and program

You will need a [NUCLEO-F042K6] board to run the firmware.

[Install Rust](https://www.rust-lang.org/tools/install):

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install cargo-embed:

    cargo install cargo-embed

Clone the repository:

    git clone https://github.com/Disasm/f042-ftdi
    cd f042-ftdi

Connect your NUCLEO-F042K6 with a USB cable.

Build and run the firmware:

    cargo embed --release

[NUCLEO-F042K6]: https://www.st.com/en/evaluation-tools/nucleo-f042k6.html
[Install Rust]: https://www.rust-lang.org/tools/install
