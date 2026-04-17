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
