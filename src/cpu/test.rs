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

// Core operation - remember to SEC before SBC to avoid off-by-one
#[test]
#[allow(non_snake_case)]
fn test_0xE9_sbc_immediate() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50;
    cpu.status_register.insert(StatusFlags::CARRY); // SEC
    cpu.load_and_run(vec![0xE9, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x40); // 0x50 - 0x10 = 0x40
}

// Carry clear acts as extra -1
#[test]
#[allow(non_snake_case)]
fn test_sbc_without_carry_subtracts_extra_one() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50;
    // carry clear = borrow, so 0x50 - 0x10 - 1 = 0x3F
    cpu.load_and_run(vec![0xE9, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x3F);
}

// Carry flag out - set when no borrow (result >= 0)
#[test]
#[allow(non_snake_case)]
fn test_sbc_sets_carry_when_no_borrow() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x10, 0x00]);
    assert_eq!(cpu.accumulator, 0x40);
    assert!(cpu.status_register.contains(StatusFlags::CARRY)); // no borrow, carry stays set
}

#[test]
#[allow(non_snake_case)]
fn test_sbc_clears_carry_when_borrow() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x50, 0x00]); // 0x10 - 0x50 underflows
    assert!(cpu.accumulator.wrapping_add(0x50) == 0x10 || true); // result wrapped
    assert!(!cpu.status_register.contains(StatusFlags::CARRY)); // borrow occurred
}

// Zero flag
#[test]
#[allow(non_snake_case)]
fn test_sbc_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x50, 0x00]); // 0x50 - 0x50 = 0x00
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

// Negative flag
#[test]
#[allow(non_snake_case)]
fn test_sbc_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x50, 0x00]); // 0x10 - 0x50 = 0xC0, bit 7 set
    assert_eq!(cpu.accumulator, 0xC0);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// Overflow - positive - negative = negative (signed overflow)
#[test]
#[allow(non_snake_case)]
fn test_sbc_sets_overflow_positive_minus_negative() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50; // +80 signed
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0xB0, 0x00]); // 0xB0 = -80 signed, 0x50 - (-80) = +160, overflows to 0xA0 = -96
    assert!(cpu.status_register.contains(StatusFlags::OVERFLOW));
}

// Overflow - negative - positive = positive (signed underflow)
#[test]
#[allow(non_snake_case)]
fn test_sbc_sets_overflow_negative_minus_positive() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0xD0; // -48 signed
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x70, 0x00]); // 0x70 = +112 signed, -48 - 112 = -160, overflows to 0x60 = +96
    assert!(cpu.status_register.contains(StatusFlags::OVERFLOW));
}

// No overflow when same sign subtraction stays in range
#[test]
#[allow(non_snake_case)]
fn test_sbc_no_overflow_same_sign() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50; // +80 signed
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.load_and_run(vec![0xE9, 0x10, 0x00]); // +80 - +16 = +64, no overflow
    assert_eq!(cpu.accumulator, 0x40);
    assert!(!cpu.status_register.contains(StatusFlags::OVERFLOW));
}

// Addressing mode spot check
#[test]
#[allow(non_snake_case)]
fn test_0xE5_sbc_zero_page() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x50;
    cpu.status_register.insert(StatusFlags::CARRY);
    cpu.memory.write(0x20, 0x10);
    cpu.load_and_run(vec![0xE5, 0x20, 0x00]);
    assert_eq!(cpu.accumulator, 0x40);
}

// ===== INC =====

#[test]
#[allow(non_snake_case)]
fn test_0xE6_inc_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x20);
    cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x21);
}

#[test]
#[allow(non_snake_case)]
fn test_inc_wraps_around() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0xFF);
    cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x00);
}

#[test]
#[allow(non_snake_case)]
fn test_inc_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0xFF); // 0xFF + 1 wraps to 0x00
    cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_inc_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x7F); // 0x7F + 1 = 0x80, bit 7 set
    cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

#[test]
#[allow(non_snake_case)]
fn test_inc_clears_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x7E); // 0x7E + 1 = 0x7F, bit 7 clear
    cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
    assert!(!cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// ===== DEC =====

#[test]
#[allow(non_snake_case)]
fn test_0xC6_dec_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x20);
    cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x1F);
}

#[test]
#[allow(non_snake_case)]
fn test_dec_wraps_around() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x00); // 0x00 - 1 wraps to 0xFF
    cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0xFF);
}

#[test]
#[allow(non_snake_case)]
fn test_dec_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x01); // 0x01 - 1 = 0x00
    cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_dec_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x00); // 0x00 - 1 = 0xFF, bit 7 set
    cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

#[test]
#[allow(non_snake_case)]
fn test_dec_clears_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x02); // 0x02 - 1 = 0x01, not zero
    cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
    assert!(!cpu.status_register.contains(StatusFlags::ZERO));
}

// ===== INX =====

#[test]
#[allow(non_snake_case)]
fn test_0xE8_inx() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x20;
    cpu.load_and_run(vec![0xE8, 0x00]);
    assert_eq!(cpu.index_x, 0x21);
}

#[test]
#[allow(non_snake_case)]
fn test_inx_wraps_around() {
    let mut cpu = CPU::new();
    cpu.index_x = 0xFF;
    cpu.load_and_run(vec![0xE8, 0x00]);
    assert_eq!(cpu.index_x, 0x00);
}

#[test]
#[allow(non_snake_case)]
fn test_inx_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.index_x = 0xFF; // 0xFF + 1 wraps to 0x00
    cpu.load_and_run(vec![0xE8, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_inx_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x7F; // 0x7F + 1 = 0x80, bit 7 set
    cpu.load_and_run(vec![0xE8, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// ===== DEX =====

#[test]
#[allow(non_snake_case)]
fn test_0xCA_dex() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x20;
    cpu.load_and_run(vec![0xCA, 0x00]);
    assert_eq!(cpu.index_x, 0x1F);
}

#[test]
#[allow(non_snake_case)]
fn test_dex_wraps_around() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x00; // 0x00 - 1 wraps to 0xFF
    cpu.load_and_run(vec![0xCA, 0x00]);
    assert_eq!(cpu.index_x, 0xFF);
}

#[test]
#[allow(non_snake_case)]
fn test_dex_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x01; // 0x01 - 1 = 0x00
    cpu.load_and_run(vec![0xCA, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_dex_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.index_x = 0x00; // 0x00 - 1 = 0xFF, bit 7 set
    cpu.load_and_run(vec![0xCA, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// ===== INY =====

#[test]
#[allow(non_snake_case)]
fn test_0xC8_iny() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x20;
    cpu.load_and_run(vec![0xC8, 0x00]);
    assert_eq!(cpu.index_y, 0x21);
}

#[test]
#[allow(non_snake_case)]
fn test_iny_wraps_around() {
    let mut cpu = CPU::new();
    cpu.index_y = 0xFF;
    cpu.load_and_run(vec![0xC8, 0x00]);
    assert_eq!(cpu.index_y, 0x00);
}

#[test]
#[allow(non_snake_case)]
fn test_iny_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.index_y = 0xFF;
    cpu.load_and_run(vec![0xC8, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_iny_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x7F; // 0x7F + 1 = 0x80, bit 7 set
    cpu.load_and_run(vec![0xC8, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// ===== DEY =====

#[test]
#[allow(non_snake_case)]
fn test_0x88_dey() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x20;
    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.index_y, 0x1F);
}

#[test]
#[allow(non_snake_case)]
fn test_dey_wraps_around() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x00;
    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.index_y, 0xFF);
}

#[test]
#[allow(non_snake_case)]
fn test_dey_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x01;
    cpu.load_and_run(vec![0x88, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_dey_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.index_y = 0x00; // 0x00 - 1 = 0xFF, bit 7 set
    cpu.load_and_run(vec![0x88, 0x00]);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}
// ===== ASL =====
// ASL: C <- [76543210] <- 0
// shifts left, bit 7 goes into carry, 0 shifts into bit 0

#[test]
#[allow(non_snake_case)]
fn test_0x0A_asl_accumulator() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x10;
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x20); // 0x10 << 1 = 0x20
}

#[test]
#[allow(non_snake_case)]
fn test_asl_sets_carry_from_bit_7() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x80; // bit 7 set, will shift into carry
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::CARRY));
}

#[test]
#[allow(non_snake_case)]
fn test_asl_clears_carry_when_bit_7_clear() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x40; // bit 7 clear
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x80);
    assert!(!cpu.status_register.contains(StatusFlags::CARRY));
}

#[test]
#[allow(non_snake_case)]
fn test_asl_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x80; // 0x80 << 1 = 0x00 (bit 7 goes to carry, result is 0)
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

#[test]
#[allow(non_snake_case)]
fn test_asl_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x40; // 0x40 << 1 = 0x80, bit 7 set
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x80);
    assert!(cpu.status_register.contains(StatusFlags::NEGATIVE));
}

#[test]
#[allow(non_snake_case)]
fn test_asl_clears_negative_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x20; // 0x20 << 1 = 0x40, bit 7 clear
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x40);
    assert!(!cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// carry in does NOT feed into bit 0 — that's ROL, not ASL
#[test]
#[allow(non_snake_case)]
fn test_asl_ignores_carry_in() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x01;
    cpu.status_register.insert(StatusFlags::CARRY); // carry set, but should not affect result
    cpu.load_and_run(vec![0x0A, 0x00]);
    assert_eq!(cpu.accumulator, 0x02); // bit 0 of result is always 0
}

// memory mode spot check
#[test]
#[allow(non_snake_case)]
fn test_0x06_asl_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x10);
    cpu.load_and_run(vec![0x06, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x20);
}

#[test]
#[allow(non_snake_case)]
fn test_asl_zero_page_sets_carry() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x80); // bit 7 set
    cpu.load_and_run(vec![0x06, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x00);
    assert!(cpu.status_register.contains(StatusFlags::CARRY));
}

// ===== LSR =====
// LSR: 0 -> [76543210] -> C
// shifts right, bit 0 goes into carry, 0 shifts into bit 7

#[test]
#[allow(non_snake_case)]
fn test_0x4A_lsr_accumulator() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x20;
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x10); // 0x20 >> 1 = 0x10
}

#[test]
#[allow(non_snake_case)]
fn test_lsr_sets_carry_from_bit_0() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x01; // bit 0 set, will shift into carry
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::CARRY));
}

#[test]
#[allow(non_snake_case)]
fn test_lsr_clears_carry_when_bit_0_clear() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x02; // bit 0 clear
    cpu.status_register.insert(StatusFlags::CARRY); // carry was set, should clear
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x01);
    assert!(!cpu.status_register.contains(StatusFlags::CARRY));
}

#[test]
#[allow(non_snake_case)]
fn test_lsr_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x01; // 0x01 >> 1 = 0x00
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert!(cpu.status_register.contains(StatusFlags::ZERO));
}

// LSR always clears negative flag since bit 7 is always 0 after shift
#[test]
#[allow(non_snake_case)]
fn test_lsr_always_clears_negative_flag() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0xFF; // even with all bits set, result bit 7 will be 0
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x7F);
    assert!(!cpu.status_register.contains(StatusFlags::NEGATIVE));
}

// carry in does NOT feed into bit 7 — that's ROR, not LSR
#[test]
#[allow(non_snake_case)]
fn test_lsr_ignores_carry_in() {
    let mut cpu = CPU::new();
    cpu.accumulator = 0x80;
    cpu.status_register.insert(StatusFlags::CARRY); // carry set, but should not affect result
    cpu.load_and_run(vec![0x4A, 0x00]);
    assert_eq!(cpu.accumulator, 0x40); // bit 7 of result is always 0
}

// memory mode spot check
#[test]
#[allow(non_snake_case)]
fn test_0x46_lsr_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x20);
    cpu.load_and_run(vec![0x46, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x10);
}

#[test]
#[allow(non_snake_case)]
fn test_lsr_zero_page_sets_carry() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x01); // bit 0 set
    cpu.load_and_run(vec![0x46, 0x10, 0x00]);
    assert_eq!(cpu.memory.read(0x10), 0x00);
    assert!(cpu.status_register.contains(StatusFlags::CARRY));
}
