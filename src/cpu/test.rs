use super::*;

#[test]
#[allow(non_snake_case)]
fn test_0xA9_lda_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x55, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA5_lda_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x55);
    cpu.load_and_run(vec![0xA5, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB5_lda_zero_page_x() {
    let mut cpu = CPU::new();
    // arg=0x10, X=0x05, reads from (0x10 + 0x05) % 256 = 0x15
    cpu.index_x = 0x05;
    cpu.memory.write(0x15, 0x55);
    cpu.load_and_run(vec![0xB5, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB5_lda_zero_page_x_wraps() {
    let mut cpu = CPU::new();
    // (0xF0 + 0x20) % 256 = 0x10, should wrap around zero page
    cpu.index_x = 0x20;
    cpu.memory.write(0x10, 0x55);
    cpu.load_and_run(vec![0xB5, 0xF0, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xAD_lda_absolute() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x2040, 0x55);
    cpu.load_and_run(vec![0xAD, 0x40, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xBD_lda_absolute_x() {
    let mut cpu = CPU::new();
    // base=0x2000, X=0x05, reads from 0x2005
    cpu.index_x = 0x05;
    cpu.memory.write(0x2005, 0x55);
    cpu.load_and_run(vec![0xBD, 0x00, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xBD_lda_absolute_x_page_cross() {
    let mut cpu = CPU::new();
    // base=0x20F0, X=0x10, crosses page boundary into 0x2100
    cpu.index_x = 0x10;
    cpu.memory.write(0x2100, 0x55);
    cpu.load_and_run(vec![0xBD, 0xF0, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB9_lda_absolute_y() {
    let mut cpu = CPU::new();
    // base=0x2000, Y=0x05, reads from 0x2005
    cpu.index_y = 0x05;
    cpu.memory.write(0x2005, 0x55);
    cpu.load_and_run(vec![0xB9, 0x00, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB9_lda_absolute_y_page_cross() {
    let mut cpu = CPU::new();
    // base=0x20F0, Y=0x10, crosses page boundary into 0x2100
    cpu.index_y = 0x10;
    cpu.memory.write(0x2100, 0x55);
    cpu.load_and_run(vec![0xB9, 0xF0, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA1_lda_indirect_x() {
    let mut cpu = CPU::new();
    // arg=0x10, X=0x04
    // pointer address = (0x10 + 0x04) % 256 = 0x14
    // pointer at 0x14/0x15 => 0x3050
    // value at 0x3050 = 0x55
    cpu.index_x = 0x04;
    cpu.memory.write(0x14, 0x50);
    cpu.memory.write(0x15, 0x30);
    cpu.memory.write(0x3050, 0x55);
    cpu.load_and_run(vec![0xA1, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA1_lda_indirect_x_wraps() {
    let mut cpu = CPU::new();
    // (0xFF + 0x02) % 256 = 0x01
    // pointer at 0x01/0x02 => 0x4000
    // value at 0x4000 = 0x55
    cpu.index_x = 0x02;
    cpu.memory.write(0x01, 0x00);
    cpu.memory.write(0x02, 0x40);
    cpu.memory.write(0x4000, 0x55);
    cpu.load_and_run(vec![0xA1, 0xFF, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB1_lda_indirect_y() {
    let mut cpu = CPU::new();
    // arg=0x10
    // pointer at 0x10/0x11 => base 0x3050
    // final address = 0x3050 + Y(0x04) = 0x3054
    // value at 0x3054 = 0x55
    cpu.memory.write(0x10, 0x50);
    cpu.memory.write(0x11, 0x30);
    cpu.index_y = 0x04;
    cpu.memory.write(0x3054, 0x55);
    cpu.load_and_run(vec![0xB1, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xB1_lda_indirect_y_page_cross() {
    let mut cpu = CPU::new();
    // pointer at 0x10/0x11 => base 0x30F0
    // final address = 0x30F0 + Y(0x10) = 0x3100 (crosses page)
    // value at 0x3100 = 0x55
    cpu.memory.write(0x10, 0xF0);
    cpu.memory.write(0x11, 0x30);
    cpu.index_y = 0x10;
    cpu.memory.write(0x3100, 0x55);
    cpu.load_and_run(vec![0xB1, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA9_lda_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_0xA9_lda_clears_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x01, 0x00]);
    assert!(!cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_0xA9_lda_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x80, 0x00]); // 0x80 has bit 7 set
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

#[test]
#[allow(non_snake_case)]
fn test_0xA9_lda_clears_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x01, 0x00]);
    assert!(!cpu.status_register.contains(StatusFlags::NEGATIVE));
}

#[test]
#[allow(non_snake_case)]
fn test_0x85_sta_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x55, 0x85, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA2_ldx_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x55, 0x00]);
    assert_eq!(cpu.index_x, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0x86_stx_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x55, 0x86, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA0_ldy_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x55, 0x00]);
    assert_eq!(cpu.index_y, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0x84_sty_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x55, 0x84, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xAA_tax() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x55, 0xAA, 0x00]);
    assert_eq!(cpu.index_x, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0x8A_txa() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x55, 0x8A, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0xA8_tay() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x55, 0xA8, 0x00]);
    assert_eq!(cpu.index_y, 0x55);
}

#[test]
#[allow(non_snake_case)]
fn test_0x98_tya() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x55, 0x98, 0x00]);
    assert_eq!(cpu.accumulator, 0x55);
}

// Core operation
#[test]
#[allow(non_snake_case)]
fn test_0x69_adc_immediate() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.load_and_run(vec![0x69, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x30);
}

// Carry flag in
#[test]
#[allow(non_snake_case)]
fn test_adc_with_carry_in() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0x69, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x31); // 0x10 + 0x20 + carry(1)
}

// Carry flag out
#[test]
#[allow(non_snake_case)]
fn test_adc_sets_carry_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0xFF;
    cpu.load_and_run(vec![0x69, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 0x00); // wraps around
    assert!(cpu.status_register.contains(StatusFlags::CARRY));
}

#[test]
#[allow(non_snake_case)]
fn test_adc_clears_carry_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x01;
    cpu.status_register.insert(StatusFlags::CARRY); // carry was set from before
    cpu.load_and_run(vec![0x69, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 0x03);
    assert!(!cpu.status_register.contains(StatusFlags::CARRY)); // no overflow, should clear
}

// Zero flag
#[test]
#[allow(non_snake_case)]
fn test_adc_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0xFF;
    cpu.load_and_run(vec![0x69, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

// Negative flag
#[test]
#[allow(non_snake_case)]
fn test_adc_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x40;
    cpu.load_and_run(vec![0x69, 0x40, 0x00]);
    assert_eq!(cpu.accumulator, 0x80); // bit 7 set
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// Overflow flag - positive + positive = negative (signed overflow)
#[test]
#[allow(non_snake_case)]
fn test_adc_sets_overflow_flag_positive_overflow() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50; // +80 signed
    cpu.load_and_run(vec![0x69, 0x50, 0x00]); // +80 signed, result = 0xA0 = -96 signed
    assert!(cpu.status_register.contains(StatusFlags::OVERFLOW));
}

// Overflow flag - negative + negative = positive (signed underflow)
#[test]
#[allow(non_snake_case)]
fn test_adc_sets_overflow_flag_negative_overflow() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0xD0; // -48 signed
    cpu.load_and_run(vec![0x69, 0xD0, 0x00]); // -48 signed, result = 0xA0... wait, 0xD0+0xD0=0x1A0
    // 0xA0 = -96 signed, both inputs negative, result negative — no overflow
    // let's use 0x90 + 0x90 instead: -112 + -112 = 0x120, truncated = 0x20 = +32, sign flipped
    assert!(!cpu.status_register.contains(StatusFlags::OVERFLOW));
}

#[test]
#[allow(non_snake_case)]
fn test_adc_sets_overflow_flag_negative_plus_negative() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x90; // -112 signed
    cpu.load_and_run(vec![0x69, 0x90, 0x00]); // result = 0x120, truncated = 0x20 = +32 signed
    assert!(cpu.status_register.contains(StatusFlags::OVERFLOW));
    assert!(cpu.status_register.contains(StatusFlags::CARRY)); // also overflowed unsigned
}

// Overflow should NOT be set when signs differ (can never overflow)
#[test]
#[allow(non_snake_case)]
fn test_adc_no_overflow_when_signs_differ() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50; // +80 signed
    cpu.load_and_run(vec![0x69, 0xD0, 0x00]); // 0xD0 = -48 signed, result = 0x120 -> 0x20
    assert!(!cpu.status_register.contains(StatusFlags::OVERFLOW));
}

// Addressing mode spot check - just one to confirm wiring
#[test]
#[allow(non_snake_case)]
fn test_0x65_adc_zero_page() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.memory.write(0x20, 0x30);
    cpu.load_and_run(vec![0x65, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x40);
}
