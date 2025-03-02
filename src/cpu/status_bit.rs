#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum StatusBit {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    Overflow = 6,
    Negative = 7,
}
