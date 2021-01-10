pub struct Hardware;

impl Hardware {
    pub fn mpsse_set_gpio_value(&self, direction: u8, value: u8) {
        // TODO
    }

    // Set `divisor` for 12MHz clock
    pub fn mpsse_set_tck_divisor(&self, divisor: u32) {
        // TODO
    }

    // [0x1B] Clock Data Bits Out on -ve clock edge LSB first (no read)
    // CLK starts at 0
    pub fn mpsse_write_tdi_bits_lsb_mode0(&self, byte: u8, nbits: u8) {
    }

    // [0x39] Clock Data Bytes In and Out LSB first
    // Out on -ve edge, in on +ve edge
    pub fn mpsse_transfer_tdi_bytes_lsb_mode0(&self, rx_bytes: &[u8], tx_bytes: &mut [u8]) {
    }

    // [0x3B] Clock Data Bits In and Out LSB first
    // Out on -ve edge, in on +ve edge
    pub fn mpsse_transfer_tdi_bits_lsb_mode0(&self, byte: u8, nbits: u8) -> u8 {
        // TODO
        0x00
    }

    // [0x4B] Clock Data to TMS pin (no read)
    // TMS with LSB first on -ve clk edge - use if clk is set to '0'
    // Bit 7 of the Byte1 is passed on to TDI/DO before the first clk of TMS
    // and is held static for the duration of TMS clocking.
    pub fn mpsse_write_tms_bits_lsb_mode0(&self, byte: u8, nbits: u8) {
        // TODO
    }

    // [0x6B] Clock Data to TMS pin with read
    // TMS with LSB first on -ve clk edge, read on +ve edge - use if clk is set to '0'
    // Bit 7 of the Byte1 is passed on to TDI/DO before the first clk of TMS
    // and is held static for the duration of TMS clocking. The TDO/DI pin is
    // sampled for the duration of TMS and a byte containing the data is passed
    // back at the end of TMS clocking.
    pub fn mpsse_transfer_tms_bits_lsb_mode0(&self, byte: u8, nbits: u8) -> u8 {
        // TODO
        0x00
    }

    pub fn serial_set_baud_rate(&self, baud_rate: u32) {
        // TODO
    }
}
