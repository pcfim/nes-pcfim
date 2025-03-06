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

fn adding_with_carry(cpu: &mut CPU, value_to_add: u8) {
    let carry = (cpu.status >> StatusBit::Carry as u8) & 1;
    let sum = value_to_add as u16 + cpu.register_a as u16 + carry as u16;

    let result: u8 = (sum & 0xFF) as u8;
    let overflow_flag: u8 = (result ^ cpu.register_a) & (value_to_add ^ result) & 0x80;

    // Result to accumulator
    cpu.register_a = result;
    // Setting flags
    if overflow_flag != 0 {
        update_status_bit(cpu, StatusBit::Overflow, BitwiseOperation::Set);
    } else {
        update_status_bit(cpu, StatusBit::Overflow, BitwiseOperation::Unset);
    }
    if sum > 0xff {
        update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Set);
    } else {
        update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Unset);
    }
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn increment_memory(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_a = cpu.register_a.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_a);
}
pub fn increment_x_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn increment_y_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

pub fn decrement_memory(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_a = cpu.register_a.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_a);
}
pub fn decrement_x_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn decrement_y_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

pub fn load_accumulator(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value: u8 = cpu.memory.memory[address as usize];
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}
pub fn load_x_register(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value: u8 = cpu.memory.memory[address as usize];
    cpu.register_x = value;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn load_y_register(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value: u8 = cpu.memory.memory[address as usize];
    cpu.register_y = value;
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

pub fn transfer_accumulator_to_x(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn transfer_accumulator_to_y(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_y = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

pub fn store_accumulator(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    cpu.memory.memory[address as usize] = cpu.register_a;
}
pub fn store_x_register(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    cpu.memory.memory[address as usize] = cpu.register_x;
}
pub fn store_y_register(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    cpu.memory.memory[address as usize] = cpu.register_y;
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

pub fn add_with_carry(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let result: u8 = cpu.memory.memory[address as usize];
    adding_with_carry(cpu, result);
}

pub fn substract_with_carry(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let result: u8 = cpu.memory.memory[address as usize];
    adding_with_carry(cpu, !result);
}

pub fn transfer_stack_pointer_to_x(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.stack_pointer;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

pub fn transfer_x_to_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_a = cpu.register_x;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn transfer_y_to_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_a = cpu.register_y;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn transfer_x_to_stack_pointer(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.stack_pointer = cpu.register_x;
}

pub fn force_interruptions(_cpu: &mut CPU, _mode: &AddressingMode) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::cpu_functions;
    use crate::cpu::cpu_model::STACK_RESET;
    use crate::cpu::memory::Memory;

    // Helper function to create a new CPU instance
    const TEST_BASE_REGISTER_A: u8 = 0x05;
    const TEST_BASE_REGISTER_X: u8 = 0x0A;
    const TEST_BASE_REGISTER_Y: u8 = 0x0F;
    const TEST_BASE_PROGRAM_COUNTER: u16 = 0x2000;
    const TEST_BASE_STATUS: u8 = 0x00;
    const SAFE_MEMORY_ADDRESS: u16 = 0x0200;
    fn create_test_cpu() -> CPU {
        CPU {
            register_a: TEST_BASE_REGISTER_A,
            register_x: TEST_BASE_REGISTER_X,
            register_y: TEST_BASE_REGISTER_Y,
            status: TEST_BASE_STATUS,
            program_counter: TEST_BASE_PROGRAM_COUNTER,
            stack_pointer: STACK_RESET,
            memory: Memory::new(),
        }
    }
    fn get_bit(current_byte: u8, status_bit: StatusBit) -> u8 {
        (current_byte >> (status_bit as u8)) & 1
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

    #[test]
    fn test_increment_memory() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            increment_memory(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_a, TEST_BASE_REGISTER_A.wrapping_add(AMOUNT));
    }

    #[test]
    fn test_increment_x_register() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            increment_x_register(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_x, TEST_BASE_REGISTER_X.wrapping_add(AMOUNT));
    }

    #[test]
    fn test_increment_y_register() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            increment_y_register(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_y, TEST_BASE_REGISTER_Y.wrapping_add(AMOUNT));
    }

    #[test]
    fn test_decrement_memory() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            decrement_memory(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_a, TEST_BASE_REGISTER_A.wrapping_sub(AMOUNT));
    }

    #[test]
    fn test_decrement_x_register() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            decrement_x_register(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_x, TEST_BASE_REGISTER_X.wrapping_sub(AMOUNT));
    }

    #[test]
    fn test_decrement_y_register() {
        let mut cpu: CPU = create_test_cpu();
        let mode: AddressingMode = AddressingMode::Immediate;
        const AMOUNT: u8 = 20;
        for _ in 0..AMOUNT {
            decrement_y_register(&mut cpu, &mode);
        }
        assert_eq!(cpu.register_y, TEST_BASE_REGISTER_Y.wrapping_sub(AMOUNT));
    }

    #[test]
    fn test_load_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);

        let data_to_load: u8 = 0xff;
        cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize] = data_to_load;

        load_accumulator(&mut cpu, &AddressingMode::Absolute);
        assert_eq!(cpu.register_a, data_to_load);

        load_x_register(&mut cpu, &AddressingMode::Absolute);
        assert_eq!(cpu.register_x, data_to_load);

        load_y_register(&mut cpu, &AddressingMode::Absolute);
        assert_eq!(cpu.register_y, data_to_load);
    }

    #[test]
    fn test_transfer_accumulator_to_x() {
        let mut cpu: CPU = create_test_cpu();
        transfer_accumulator_to_x(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_x, TEST_BASE_REGISTER_A);
    }

    #[test]
    fn test_transfer_accumulator_to_y() {
        let mut cpu: CPU = create_test_cpu();
        transfer_accumulator_to_y(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_y, TEST_BASE_REGISTER_A);
    }

    #[test]
    fn test_transfer_x_to_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        transfer_x_to_accumulator(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_a, TEST_BASE_REGISTER_X);
    }

    #[test]
    fn test_transfer_y_to_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        transfer_y_to_accumulator(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_a, TEST_BASE_REGISTER_Y);
    }

    #[test]
    fn test_transfer_stack_pointer_to_x() {
        let mut cpu: CPU = create_test_cpu();
        transfer_stack_pointer_to_x(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.register_x, STACK_RESET);
    }

    #[test]
    fn test_transfer_x_to_stack_pointer() {
        let mut cpu: CPU = create_test_cpu();
        transfer_x_to_stack_pointer(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.stack_pointer, TEST_BASE_REGISTER_X);
    }

    #[test]
    fn test_store_accumulator() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);
        store_accumulator(&mut cpu, &AddressingMode::Absolute);
        let value_stored = cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize];
        assert_eq!(value_stored, TEST_BASE_REGISTER_A);
    }

    #[test]
    fn test_store_x_register() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);
        store_x_register(&mut cpu, &AddressingMode::Absolute);
        let value_stored = cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize];
        assert_eq!(value_stored, TEST_BASE_REGISTER_X);
    }

    #[test]
    fn test_store_y_register() {
        let mut cpu: CPU = create_test_cpu();
        cpu.memory
            .write_u16(TEST_BASE_PROGRAM_COUNTER, SAFE_MEMORY_ADDRESS);
        store_y_register(&mut cpu, &AddressingMode::Absolute);
        let value_stored = cpu.memory.memory[SAFE_MEMORY_ADDRESS as usize];
        assert_eq!(value_stored, TEST_BASE_REGISTER_Y);
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
    type AddOrSubstractWithCarryTests = (u8, u8, bool, u8, u8, u8, u8, u8);
    fn generate_tests_add_with_carry() -> Vec<AddOrSubstractWithCarryTests> {
        // accum_init, memory, carry_initial,
        // C, Z, N, V
        let lst: Vec<AddOrSubstractWithCarryTests> = vec![
            (0x60, 0x70, false, 0xd0, 0, 0, 1, 1),
            (0x7F, 0x01, false, 0x80, 0, 0, 1, 1),
            (0x80, 0xFF, false, 0x7F, 1, 0, 0, 1),
            // Sign change positive to negative with overflow (0x7F + 0x01 = 0x80)
            (0x7F, 0x01, false, 0x80, 0, 0, 1, 1),
            // Sign change negative to positive with overflow and carry (0x80 + 0xFF = 0x7F with carry)
            (0x80, 0xFF, false, 0x7F, 1, 0, 0, 1),
            // Carry generation without overflow (0xFF + 0x01 = 0x00 with carry)
            (0xFF, 0x01, false, 0x00, 1, 1, 0, 0),
            // Complex case with carry-in (0x80 + 0x80 + 1 = 0x01 with carry)
            (0x80, 0x80, true, 0x01, 1, 0, 0, 1),
            // Zero result with carry-in (0xFF + 0x00 + 1 = 0x00 with carry)
            (0xFF, 0x00, true, 0x00, 1, 1, 0, 0),
            (0x80, 0x80, true, 0x01, 1, 0, 0, 1),
        ];
        lst
    }

    fn generate_tests_substract_with_carry() -> Vec<AddOrSubstractWithCarryTests> {
        // accum_init, memory, carry_initial,
        // C, Z, N, V
        let lst: Vec<AddOrSubstractWithCarryTests> = vec![
            (0x05, 0x01, false, 0x03, 1, 0, 0, 0),
            (0x00, 0x01, true, 0xff, 0, 0, 1, 0),
            (0x7f, 0xff, true, 0x80, 0, 0, 1, 1),
            (0x40, 0xc0, true, 0x80, 0, 0, 1, 1),
            (0x50, 0x60, true, 0xf0, 0, 0, 1, 0),
            // Sign change negative to positive with overflow (0x80 - 0x01 = 0x7F)
            (0x80, 0x01, true, 0x7F, 1, 0, 0, 1),
            // Borrow generation (0x00 - 0x01 = 0xFF with borrow)
            (0x00, 0x01, true, 0xFF, 0, 0, 1, 0),
            // Sign change positive to negative with overflow (0x7F - 0xFF = 0x80)
            (0x7F, 0xFF, true, 0x80, 0, 0, 1, 1),
            // Zero result (0x01 - 0x01 = 0x00 without borrow)
            (0x01, 0x01, true, 0x00, 1, 1, 0, 0),
            // Borrow-in affects result (0x00 - 0x00 with borrow-in = 0xFF)
            (0x00, 0x00, false, 0xFF, 0, 0, 1, 0),
            (0x80, 0x7F, false, 0x00, 1, 1, 0, 1),
        ];
        lst
    }
    #[test]
    fn testing_add_with_carry() {
        for testing_parameters in generate_tests_add_with_carry() {
            let mut cpu: CPU = create_test_cpu();
            cpu.register_a = testing_parameters.0;
            cpu.memory.memory[TEST_BASE_PROGRAM_COUNTER as usize] = testing_parameters.1;
            if testing_parameters.2 {
                update_status_bit(&mut cpu, StatusBit::Carry, BitwiseOperation::Set);
            } else {
                update_status_bit(&mut cpu, StatusBit::Carry, BitwiseOperation::Unset);
            }
            add_with_carry(&mut cpu, &AddressingMode::Immediate);
            // Summing 0x60 + 0x50
            assert_eq!(cpu.register_a, testing_parameters.3);
            assert_eq!(get_bit(cpu.status, StatusBit::Carry), testing_parameters.4);
            assert_eq!(get_bit(cpu.status, StatusBit::Zero), testing_parameters.5);
            assert_eq!(
                get_bit(cpu.status, StatusBit::Negative),
                testing_parameters.6
            );
            assert_eq!(
                get_bit(cpu.status, StatusBit::Overflow),
                testing_parameters.7
            );
        }
    }

    #[test]
    fn testing_substract_with_carry() {
        for testing_parameters in generate_tests_substract_with_carry() {
            let mut cpu: CPU = create_test_cpu();
            cpu.register_a = testing_parameters.0;
            cpu.memory.memory[TEST_BASE_PROGRAM_COUNTER as usize] = testing_parameters.1;
            if testing_parameters.2 {
                update_status_bit(&mut cpu, StatusBit::Carry, BitwiseOperation::Set);
            } else {
                update_status_bit(&mut cpu, StatusBit::Carry, BitwiseOperation::Unset);
            }
            substract_with_carry(&mut cpu, &AddressingMode::Immediate);
            // Summing 0x60 + 0x50
            assert_eq!(cpu.register_a, testing_parameters.3);
            assert_eq!(get_bit(cpu.status, StatusBit::Carry), testing_parameters.4);
            assert_eq!(get_bit(cpu.status, StatusBit::Zero), testing_parameters.5);
            assert_eq!(
                get_bit(cpu.status, StatusBit::Negative),
                testing_parameters.6
            );
            assert_eq!(
                get_bit(cpu.status, StatusBit::Overflow),
                testing_parameters.7
            );
        }
    }
}
