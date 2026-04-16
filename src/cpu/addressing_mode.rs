#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Indirect,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
}
