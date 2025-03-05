use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::bitwise_operation::BitwiseOperation;
use crate::cpu::cpu_model::CPU;
use crate::cpu::status_bit::StatusBit;
// Function helpers

pub fn update_zero_and_negative_flags(cpu: &mut CPU, result: u8) {
    if result == 0 {
        cpu.status |= 0b0000_0010;
    } else {
        cpu.status &= 0b1111_1101;
    }

    if result & 0b1000_0000 != 0 {
        cpu.status |= 0b1000_0000;
    } else {
        cpu.status &= 0b0111_1111;
    }
}

pub fn get_operand_address(cpu: &mut CPU, mode: &AddressingMode) -> u16 {
    match mode {
        AddressingMode::Immediate => cpu.program_counter,

        AddressingMode::ZeroPage => cpu.memory.memory[cpu.program_counter as usize] as u16,

        AddressingMode::Absolute => cpu.memory.read_u16(cpu.program_counter),

        AddressingMode::ZeroPage_X => {
            let pos = cpu.memory.memory[cpu.program_counter as usize];
            pos.wrapping_add(cpu.register_x) as u16
        }
        AddressingMode::ZeroPage_Y => {
            let pos = cpu.memory.memory[cpu.program_counter as usize];
            pos.wrapping_add(cpu.register_y) as u16
        }

        AddressingMode::Absolute_X => {
            let base = cpu.memory.read_u16(cpu.program_counter);
            base.wrapping_add(cpu.register_x as u16)
        }
        AddressingMode::Absolute_Y => {
            let base = cpu.memory.read_u16(cpu.program_counter);
            base.wrapping_add(cpu.register_y as u16)
        }

        AddressingMode::Indirect_X => {
            let base = cpu.memory.memory[cpu.program_counter as usize];

            let ptr: u8 = base.wrapping_add(cpu.register_x);
            let lo = cpu.memory.memory[ptr as usize];
            let hi = cpu.memory.memory[ptr.wrapping_add(1) as usize];
            ((hi as u16) << 8) | (lo as u16)
        }
        AddressingMode::Indirect_Y => {
            let base = cpu.memory.memory[cpu.program_counter as usize];

            let lo = cpu.memory.memory[base as usize];
            let hi = cpu.memory.memory[base.wrapping_add(1) as usize];
            let deref_base = ((hi as u16) << 8) | (lo as u16);
            deref_base.wrapping_add(cpu.register_y as u16)
        }
        AddressingMode::Relative => {
            let offset = cpu.memory.memory[cpu.program_counter as usize] as i8;
            (cpu.program_counter.wrapping_add(1) as i16 + offset as i16) as u16
        }
        AddressingMode::NoneAddressing => {
            panic!("mode {:?} is not supported", mode);
        }
    }
}
fn update_status_bit(cpu: &mut CPU, position: StatusBit, op: BitwiseOperation) {
    match op {
        BitwiseOperation::Set => {
            cpu.status |= 1 << (position as u8);
        }
        BitwiseOperation::Unset => {
            // cpu.status &= 0xff ^ 1 << (position as u8);
            cpu.status &= !(1 << (position as u8));
            // cpu.status |= 1 << position
            // cpu.status ^= 1 << position
        }
        BitwiseOperation::Flip => cpu.status ^= 1 << (position as u8),
    }
}
fn compare(cpu: &mut CPU, mode: &AddressingMode, value_to_compare: u8) {
    let address = get_operand_address(cpu, mode);
    let value: u8 = cpu.memory.memory[address as usize];

    if value_to_compare >= value {
        update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Set);
    } else {
        update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Unset);
    }

    if value_to_compare == value {
        update_status_bit(cpu, StatusBit::Zero, BitwiseOperation::Set);
    } else {
        update_status_bit(cpu, StatusBit::Zero, BitwiseOperation::Unset);
    }

    let result = value_to_compare.wrapping_sub(value);
    if result & 0b1000_0000 != 0 {
        update_status_bit(cpu, StatusBit::Negative, BitwiseOperation::Set);
    } else {
        update_status_bit(cpu, StatusBit::Negative, BitwiseOperation::Unset);
    }
}

// Function implementations for CPU instructions
pub fn increment_x_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn branch(cpu: &mut CPU, condition: bool) {
    if condition {
        let offset = cpu.memory.memory[cpu.program_counter as usize] as i8;
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
}

pub fn branch_if_carry_clear(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Carry as u8)) == 0);
}

pub fn branch_if_carry_set(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Carry as u8)) != 0);
}

pub fn branch_if_equal(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Zero as u8)) != 0);
}

pub fn branch_if_minus(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Negative as u8)) != 0);
}

pub fn branch_if_not_equal(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Zero as u8)) == 0);
}

pub fn branch_if_positive(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Negative as u8)) == 0);
}

pub fn branch_if_overflow_clear(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Overflow as u8)) == 0);
}

pub fn branch_if_overflow_set(cpu: &mut CPU, _mode: &AddressingMode) {
    branch(cpu, cpu.status & (1 << (StatusBit::Overflow as u8)) != 0);
}

pub fn load_accumulator(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value: u8 = cpu.memory.memory[address as usize];
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn transfer_accumulator_to_x(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

pub fn store_accumulator(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    cpu.memory.memory[address as usize] = cpu.register_a;
}
pub fn compare_a(cpu: &mut CPU, mode: &AddressingMode) {
    compare(cpu, mode, cpu.register_a);
}
pub fn compare_x(cpu: &mut CPU, mode: &AddressingMode) {
    compare(cpu, mode, cpu.register_x);
}
pub fn compare_y(cpu: &mut CPU, mode: &AddressingMode) {
    compare(cpu, mode, cpu.register_y);
}
pub fn force_interruptions(_cpu: &mut CPU, _mode: &AddressingMode) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::cpu_functions;
    use crate::cpu::memory::Memory;

    // Helper function to create a new CPU instance
    fn create_test_cpu() -> CPU {
        CPU {
            register_a: 0x05,
            register_x: 0x0A,
            register_y: 0x0F,
            status: 0x00,
            program_counter: 0x2000,
            memory: Memory::new(),
        }
    }
    #[test]
    fn test_immediate() {
        let mut cpu: CPU = create_test_cpu();

        let mode: AddressingMode = AddressingMode::Immediate;

        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            cpu.program_counter
        );
    }

    #[test]
    fn test_zero_page() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode: AddressingMode = AddressingMode::ZeroPage;

        assert_eq!(cpu_functions::get_operand_address(&mut cpu, &mode), data);
    }

    #[test]
    fn test_zero_page_x() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode: AddressingMode = AddressingMode::ZeroPage_X;

        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            data + cpu.register_x as u16
        );
    }

    #[test]
    fn test_zero_page_y() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);

        let mode = AddressingMode::ZeroPage_Y;
        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            data + cpu.register_y as u16
        );
    }

    #[test]
    fn test_get_operand_address_relative_positive() {
        let mut cpu: CPU = create_test_cpu();
        cpu.program_counter = 0x2000;
        cpu.memory.memory[cpu.program_counter as usize] = 0x05;
        let mode: AddressingMode = AddressingMode::Relative;
        let effective_address: u16 = cpu_functions::get_operand_address(&mut cpu, &mode);
        assert_eq!(effective_address, 0x2006);
    }

    #[test]
    fn test_get_operand_address_relative_negative() {
        let mut cpu: CPU = create_test_cpu();
        cpu.program_counter = 0x2000;
        cpu.memory.memory[cpu.program_counter as usize] = 0xFB;
        let mode: AddressingMode = AddressingMode::Relative;
        let effective_address: u16 = cpu_functions::get_operand_address(&mut cpu, &mode);
        assert_eq!(effective_address, 0x1FFC);
    }

    #[test]
    fn test_get_operand_address_absolute() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute;

        assert_eq!(cpu_functions::get_operand_address(&mut cpu, &mode), data);
    }

    #[test]
    fn test_get_operand_address_absolute_x() {
        let mut cpu: CPU = create_test_cpu();

        let data = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute_X;

        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            data + cpu.register_x as u16
        );
    }

    #[test]
    fn test_get_operand_address_absolute_y() {
        let mut cpu: CPU = create_test_cpu();

        let data = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute_Y;

        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            data + cpu.register_y as u16
        );
    }

    #[test]
    fn test_get_operand_address_indirect_x() {
        let mut cpu: CPU = create_test_cpu();

        let base: u8 = 0x20;
        let ptr: u8 = base.wrapping_add(cpu.register_x);
        cpu.memory.memory[cpu.program_counter as usize] = base;
        cpu.memory.memory[ptr as usize] = 0x34;
        cpu.memory.memory[ptr.wrapping_add(1) as usize] = 0x12;

        let mode = AddressingMode::Indirect_X;
        assert_eq!(cpu_functions::get_operand_address(&mut cpu, &mode), 0x1234);
    }

    #[test]
    fn test_get_operand_address_indirect_y() {
        let mut cpu: CPU = create_test_cpu();

        let base: u8 = 0x40;
        cpu.memory.memory[cpu.program_counter as usize] = base;
        cpu.memory.memory[base as usize] = 0x78;
        cpu.memory.memory[base.wrapping_add(1) as usize] = 0x56;

        let mode = AddressingMode::Indirect_Y;
        assert_eq!(
            cpu_functions::get_operand_address(&mut cpu, &mode),
            0x5678 + cpu.register_y as u16
        );
    }

    #[test]
    #[should_panic]
    fn test_get_operand_address_none_addressing() {
        let mut cpu: CPU = create_test_cpu();
        let mode = AddressingMode::NoneAddressing;
        cpu_functions::get_operand_address(&mut cpu, &mode);
    }

    // Tests for the functions themselves

    const SAFE_MEMORY_ADDRESS: u16 = 0x0200;
    const TEST_BASE_PROGRAM_COUNTER: u16 = 0x2000;
    #[test]
    fn test_increment_x_register_by_1() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        increment_x_register(&mut cpu, &mode);
        assert_eq!(cpu.register_x, 0x0B);
    }

    #[test]
    fn test_load_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);
        cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize] = 0xff;

        load_accumulator(&mut cpu, &AddressingMode::Absolute);
        assert_eq!(cpu.register_a, 0xff);
    }

    #[test]
    fn test_transfer_accumulator_to_x() {
        let mut cpu: CPU = create_test_cpu();
        transfer_accumulator_to_x(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_x, 0x05);
    }

    #[test]
    fn test_store_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);
        store_accumulator(&mut cpu, &AddressingMode::Absolute);
        let value_stored = cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize];
        assert_eq!(value_stored, 0x05);
    }

    #[test]
    fn test_compare_equal() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory.memory[TEST_BASE_PROGRAM_COUNTER as usize] = 0x40;
        compare(&mut cpu, &AddressingMode::Immediate, 0x40);
        let status: u8 = cpu.status;
        assert_eq!(status, 0x03);
    }
    #[test]
    fn test_compare_lesser() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory.memory[TEST_BASE_PROGRAM_COUNTER as usize] = 0xff;
        compare(&mut cpu, &AddressingMode::Immediate, 0x01);
        let status: u8 = cpu.status;
        assert_eq!(status, 0x00);
    }

    #[test]
    fn test_compare_greater() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory.memory[TEST_BASE_PROGRAM_COUNTER as usize] = 0x80;
        compare(&mut cpu, &AddressingMode::Immediate, 0x7f);
        let status: u8 = cpu.status;
        assert_eq!(status, 0x80);
    }

    // --- Tests for branch_if_carry_clear ---

    #[test]
    fn test_branch_if_carry_clear_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_carry_clear(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_carry_clear_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x01;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_carry_clear(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_carry_set ---

    #[test]
    fn test_branch_if_carry_set_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x01;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_carry_set(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_carry_set_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_carry_set(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_equal ---

    #[test]
    fn test_branch_if_equal_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x02;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_equal(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_equal_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_equal(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_minus ---

    #[test]
    fn test_branch_if_minus_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x80;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_minus(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_minus_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_minus(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_not_equal ---

    #[test]
    fn test_branch_if_not_equal_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_not_equal(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_not_equal_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x02;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_not_equal(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_positive ---

    #[test]
    fn test_branch_if_positive_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_positive(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_positive_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x80;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_positive(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_overflow_clear ---

    #[test]
    fn test_branch_if_overflow_clear_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_overflow_clear(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_overflow_clear_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x40;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_overflow_clear(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }

    // --- Tests for branch_if_overflow_set ---

    #[test]
    fn test_branch_if_overflow_set_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x40;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_overflow_set(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000u16.wrapping_add(5));
    }

    #[test]
    fn test_branch_if_overflow_set_not_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_overflow_set(&mut cpu, &AddressingMode::Relative);
        assert_eq!(cpu.program_counter, 0x1000);
    }
}
