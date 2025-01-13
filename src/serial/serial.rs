pub trait Serial {
    fn init() -> &'static Self;
    fn enable(&self);
    fn disable(&self);
    fn write_byte(&self, c: u8);
    fn read_byte(&self) -> u8;
    fn write_string(&self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }
}
