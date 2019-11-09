#[derive(Debug, Default, Clone)]
pub struct Services(u64);

impl Services {
    pub fn with_network(mut self, v: bool) -> Self {
        self.set_bit(0, v);
        self
    }

    fn set_bit(&mut self, bit: usize, bit_value: bool) {
        if bit_value {
            self.0 |= 1 << bit
        } else {
            self.0 &= !(1 << bit)
        }
    }
}
