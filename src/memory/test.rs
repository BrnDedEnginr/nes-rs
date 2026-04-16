use super::*;

#[test]
fn test_u8_initial_state() {
    let memory = Memory::new();
    assert_eq!(memory.read(0x0000), 0x00);
    assert_eq!(memory.read(0x7FFF), 0x00);
    assert_eq!(memory.read(0xFFFE), 0x00);
}

#[test]
fn test_u8_write_read_roundtrip() {
    let mut memory = Memory::new();
    memory.write(0x0000, 0xDE);
    assert_eq!(memory.read(0x0000), 0xDE);
}

#[test]
fn test_u8_zero() {
    let mut memory = Memory::new();
    memory.write(0x0000, 0xFF);
    memory.write(0x0000, 0x00);
    assert_eq!(memory.read(0x0000), 0x00);
}

#[test]
fn test_u8_overwrite() {
    let mut memory = Memory::new();
    memory.write(0x0010, 0xAA);
    memory.write(0x0010, 0xBB);
    assert_eq!(memory.read(0x0010), 0xBB);
}

#[test]
fn test_u8_independence() {
    let mut memory = Memory::new();
    memory.write(0x0010, 0xFF);
    assert_eq!(memory.read(0x000F), 0x00);
    assert_eq!(memory.read(0x0011), 0x00);
}

#[test]
fn test_u8_boundary_addresses() {
    let mut memory = Memory::new();
    memory.write(0x0000, 0x01);
    memory.write(0xFFFE, 0x02);
    assert_eq!(memory.read(0x0000), 0x01);
    assert_eq!(memory.read(0xFFFE), 0x02);
}

#[test]
fn test_u8_all_bit_patterns() {
    let mut memory = Memory::new();
    for val in [0x00u8, 0x01, 0x7F, 0x80, 0xFE, 0xFF] {
        memory.write(0x0020, val);
        assert_eq!(memory.read(0x0020), val);
    }
}

#[test]
fn test_u16_low_byte_at_base_address() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0xDEAD);
    assert_eq!(memory.read(0x0000), 0xAD);
    assert_eq!(memory.read(0x0001), 0xDE);
}

#[test]
fn test_u16_high_byte_zero() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0x00FF);
    assert_eq!(memory.read(0x0000), 0xFF);
    assert_eq!(memory.read(0x0001), 0x00);
}

#[test]
fn test_u16_low_byte_zero() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0xFF00);
    assert_eq!(memory.read(0x0000), 0x00);
    assert_eq!(memory.read(0x0001), 0xFF);
}

#[test]
fn test_u16_zero_value() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0x0000);
    assert_eq!(memory.read(0x0000), 0x00);
    assert_eq!(memory.read(0x0001), 0x00);
    assert_eq!(memory.read_u16(0x0000), 0x0000);
}

#[test]
fn test_u16_max_value() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0xFFFF);
    assert_eq!(memory.read(0x0000), 0xFF);
    assert_eq!(memory.read(0x0001), 0xFF);
    assert_eq!(memory.read_u16(0x0000), 0xFFFF);
}

#[test]
fn test_u16_roundtrip() {
    let mut memory = Memory::new();
    for val in [0x0000u16, 0x0001, 0x00FF, 0xFF00, 0x1234, 0xDEAD, 0xFFFF] {
        memory.write_u16(0x0000, val);
        assert_eq!(memory.read_u16(0x0000), val);
    }
}

#[test]
fn test_u16_overwrite() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0x1234);
    memory.write_u16(0x0000, 0xABCD);
    assert_eq!(memory.read_u16(0x0000), 0xABCD);
    assert_eq!(memory.read(0x0000), 0xCD);
    assert_eq!(memory.read(0x0001), 0xAB);
}

#[test]
fn test_u16_independence() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0x1234);
    memory.write_u16(0x0002, 0x5678);
    assert_eq!(memory.read_u16(0x0000), 0x1234);
    assert_eq!(memory.read_u16(0x0002), 0x5678);
    assert_eq!(memory.read(0x0000), 0x34);
    assert_eq!(memory.read(0x0001), 0x12);
    assert_eq!(memory.read(0x0002), 0x78);
    assert_eq!(memory.read(0x0003), 0x56);
}

#[test]
fn test_u16_does_not_affect_preceding_byte() {
    let mut memory = Memory::new();
    memory.write(0x0000, 0xFF);
    memory.write_u16(0x0001, 0x1234);
    assert_eq!(memory.read(0x0000), 0xFF);
}

#[test]
fn test_u16_does_not_affect_following_byte() {
    let mut memory = Memory::new();
    memory.write(0x0003, 0xFF);
    memory.write_u16(0x0001, 0x1234);
    assert_eq!(memory.read(0x0003), 0xFF);
}

#[test]
fn test_u8_write_read_as_u16() {
    let mut memory = Memory::new();
    memory.write(0x0000, 0xCD);
    memory.write(0x0001, 0xAB);
    assert_eq!(memory.read_u16(0x0000), 0xABCD);
}

#[test]
fn test_u16_write_read_as_u8() {
    let mut memory = Memory::new();
    memory.write_u16(0x0000, 0xABCD);
    assert_eq!(memory.read(0x0000), 0xCD);
    assert_eq!(memory.read(0x0001), 0xAB);
}
