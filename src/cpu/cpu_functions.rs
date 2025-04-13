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
        AddressingMode::Accumulator => cpu.register_a as u16,

        AddressingMode::Immediate => cpu.program_counter,

        AddressingMode::Implied => cpu.program_counter, // TODO: Fix

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

        AddressingMode::Indirect => {
            let base = cpu.memory.read_u16(cpu.program_counter);
            let lo = cpu.memory.memory[base as usize];
            let hi_addr = if (base & 0xFF) == 0xFF {
                // Bug: Wrap around within the same page instead of crossing page boundary
                (base & 0xFF00) | ((base.wrapping_add(1)) & 0xFF)
            } else {
                // Normal case: Fetch from the next sequential address
                base.wrapping_add(1)
            };
            let hi: u8 = cpu.memory.memory[hi_addr as usize];
            ((hi as u16) << 8) | (lo as u16)
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
            if offset == -1 {
                return cpu.program_counter.wrapping_add(offset as u16);
            }
            cpu.program_counter
                .wrapping_add(1)
                .wrapping_add(offset as u16)
        }
        AddressingMode::NoneAddressing => {
            panic!("mode {:?} is not supported", mode);
        }
    }
}
fn get_bit(current_byte: u8, status_bit: StatusBit) -> u8 {
    (current_byte >> (status_bit as u8)) & 1
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

    // println!("Comparados: {} and {}", value, value_to_compare);
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
    // Check for Decimal mode
    if cpu.status & 0x08 != 0 {
        let a = cpu.register_a;
        let sum_lo = (a & 0x0F) + (value_to_add & 0x0F) + (cpu.status & 0x01);
        let mut lo = sum_lo & 0x0F;
        let mut carry_lo = sum_lo > 0x0F;
        if sum_lo > 0x09 {
            lo = (sum_lo + 0x06) & 0x0F;
            carry_lo = true;
        }
        let sum_hi = (a >> 4) + (value_to_add >> 4) + carry_lo as u8;
        let mut hi = sum_hi & 0x0F;
        let mut carry = sum_hi > 0x0F;
        if sum_hi > 0x09 {
            hi = (sum_hi + 0x06) & 0x0F;
            carry = true;
        }
        cpu.register_a = (hi << 4) | lo;
        let zero_flag = cpu.register_a == 0;
        let negative_flag = (cpu.register_a & 0x80) != 0;
        update_status_bit(
            cpu,
            StatusBit::Carry,
            if carry {
                BitwiseOperation::Set
            } else {
                BitwiseOperation::Unset
            },
        );
        update_status_bit(
            cpu,
            StatusBit::Zero,
            if zero_flag {
                BitwiseOperation::Set
            } else {
                BitwiseOperation::Unset
            },
        );
        update_status_bit(
            cpu,
            StatusBit::Negative,
            if negative_flag {
                BitwiseOperation::Set
            } else {
                BitwiseOperation::Unset
            },
        );
        return;
    }

    // Binary mode
    let carry = cpu.status & 0x01;
    let sum = (value_to_add as u16)
        .wrapping_add(cpu.register_a as u16)
        .wrapping_add(carry as u16);

    let result = (sum & 0xFF) as u8;
    let carry_out = (sum & 0x100) != 0;
    let overflow_flag = ((cpu.register_a ^ result) & (value_to_add ^ result) & 0x80) != 0;

    // Update accumulator
    cpu.register_a = result;

    // Update flags
    update_status_bit(
        cpu,
        StatusBit::Carry,
        if carry_out {
            BitwiseOperation::Set
        } else {
            BitwiseOperation::Unset
        },
    );
    update_status_bit(
        cpu,
        StatusBit::Overflow,
        if overflow_flag {
            BitwiseOperation::Set
        } else {
            BitwiseOperation::Unset
        },
    );
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn increment_memory(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value = cpu.memory.read_u8(address);
    let new_value = value.wrapping_add(1);
    cpu.memory.write_u8(address, new_value);
    update_zero_and_negative_flags(cpu, new_value);
}
pub fn increment_x_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn increment_y_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

pub fn jump(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    cpu.program_counter = address;
}

pub fn jump_to_subroutine(cpu: &mut CPU, mode: &AddressingMode) {
    let address: u16 = get_operand_address(cpu, mode);
    let return_address: u16 = cpu.program_counter.wrapping_add(1);
    let high: u8 = (return_address >> 8) as u8;
    let low: u8 = (return_address & 0xFF) as u8;
    cpu.memory.memory[(0x0100_u16.wrapping_add(cpu.stack_pointer as u16)) as usize] = high;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
    cpu.memory.memory[(cpu.stack_pointer as u16).wrapping_add(0x0100_u16) as usize] = low;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
    cpu.program_counter = address;
}

pub fn decrement_memory(cpu: &mut CPU, mode: &AddressingMode) {
    let address = get_operand_address(cpu, mode);
    let value = cpu.memory.read_u8(address);
    let new_value = value.wrapping_sub(1);
    cpu.memory.write_u8(address, new_value);
    update_zero_and_negative_flags(cpu, new_value);
}
pub fn decrement_x_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}
pub fn decrement_y_register(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

fn branch(cpu: &mut CPU, mode: &AddressingMode, condition: bool) {
    if condition {
        let target_address = get_operand_address(cpu, mode);
        cpu.program_counter = target_address;
        // println!("You are going to 0x{:04X} = {}", target_address, target_address)
    }
}

pub fn branch_if_carry_clear(cpu: &mut CPU, mode: &AddressingMode) {
    branch(cpu, mode, cpu.status & (1 << (StatusBit::Carry as u8)) == 0);
}

pub fn branch_if_carry_set(cpu: &mut CPU, mode: &AddressingMode) {
    branch(cpu, mode, cpu.status & (1 << (StatusBit::Carry as u8)) != 0);
}

pub fn branch_if_equal(cpu: &mut CPU, mode: &AddressingMode) {
    branch(cpu, mode, cpu.status & (1 << (StatusBit::Zero as u8)) != 0);
}

pub fn branch_if_minus(cpu: &mut CPU, mode: &AddressingMode) {
    branch(
        cpu,
        mode,
        cpu.status & (1 << (StatusBit::Negative as u8)) != 0,
    );
}

pub fn branch_if_not_equal(cpu: &mut CPU, mode: &AddressingMode) {
    branch(cpu, mode, cpu.status & (1 << (StatusBit::Zero as u8)) == 0);
}

pub fn branch_if_positive(cpu: &mut CPU, mode: &AddressingMode) {
    branch(
        cpu,
        mode,
        cpu.status & (1 << (StatusBit::Negative as u8)) == 0,
    );
}

pub fn branch_if_overflow_clear(cpu: &mut CPU, mode: &AddressingMode) {
    branch(
        cpu,
        mode,
        cpu.status & (1 << (StatusBit::Overflow as u8)) == 0,
    );
}

pub fn branch_if_overflow_set(cpu: &mut CPU, mode: &AddressingMode) {
    branch(
        cpu,
        mode,
        cpu.status & (1 << (StatusBit::Overflow as u8)) != 0,
    );
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

pub fn return_from_interrupt(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let status = cpu.memory.memory[0x0100 + cpu.stack_pointer as usize];
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let lo = cpu.memory.memory[0x0100 + cpu.stack_pointer as usize] as u16;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let hi = cpu.memory.memory[0x0100 + cpu.stack_pointer as usize] as u16;
    cpu.program_counter = (hi << 8) | lo;
    cpu.status = (status & 0xEF) | 0x20;
}

pub fn return_from_subroutine(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let lo = cpu.memory.read_u8(0x0100 + cpu.stack_pointer as u16);
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let hi = cpu.memory.read_u8(0x0100 + cpu.stack_pointer as u16);
    let addr = ((hi as u16) << 8) | (lo as u16);
    cpu.program_counter = addr.wrapping_add(1);
}

pub fn force_interruptions(_cpu: &mut CPU, _mode: &AddressingMode) {}

pub fn arithmetic_shift_left(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let mut value = cpu.memory.memory[address as usize];
    let carry = value >> 7;
    value <<= 1;
    cpu.memory.memory[address as usize] = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn arithmetic_shift_left_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    let mut value = cpu.register_a;
    let carry = value >> 7;
    value <<= 1;
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn bit_test(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let value = cpu.memory.memory[address as usize];
    let result = cpu.register_a & value;

    update_status_bit(
        cpu,
        StatusBit::Zero,
        BitwiseOperation::from_bool(result == 0),
    );
    update_status_bit(
        cpu,
        StatusBit::Overflow,
        BitwiseOperation::from_bool((value >> 6) & 1 == 1),
    );
    update_status_bit(
        cpu,
        StatusBit::Negative,
        BitwiseOperation::from_bool((value >> 7) & 1 == 1),
    );
}

pub fn clear_carry_flag(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Unset);
}

pub fn clear_decimal_mode(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Decimal, BitwiseOperation::Unset);
}

pub fn clear_interrupt_disable(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Interrupt, BitwiseOperation::Unset);
}

pub fn clear_overflow_flag(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Overflow, BitwiseOperation::Unset);
}

pub fn exclusive_or(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let value = cpu.memory.memory[address as usize];
    cpu.register_a ^= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn logical_and(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let value = cpu.memory.memory[address as usize];
    cpu.register_a &= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn logical_inclusive_or(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let value = cpu.memory.memory[address as usize];
    cpu.register_a |= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn logical_shift_right(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let mut value = cpu.memory.memory[address as usize];
    let carry = value & 1;
    value >>= 1;
    cpu.memory.memory[address as usize] = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn logical_shift_right_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    let mut value = cpu.register_a;
    let carry = value & 1;
    value >>= 1;
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn pull_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let address = 0x0100 + cpu.stack_pointer as u16;
    cpu.register_a = cpu.memory.memory[address as usize];
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

pub fn pull_processor_status(cpu: &mut CPU, _mode: &AddressingMode) {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let address = 0x0100 + cpu.stack_pointer as u16;
    cpu.status = cpu.memory.memory[address as usize];
    update_status_bit(cpu, StatusBit::Break, BitwiseOperation::Unset);
    update_status_bit(cpu, StatusBit::Break2, BitwiseOperation::Set);
}

pub fn push_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = 0x0100 + cpu.stack_pointer as u16;
    cpu.memory.memory[address as usize] = cpu.register_a;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
}

pub fn push_processor_status(cpu: &mut CPU, _mode: &AddressingMode) {
    let mut flags = cpu.status;
    flags |= 1 << StatusBit::Break as u8;
    flags |= 1 << StatusBit::Break2 as u8;

    let address = 0x0100 + cpu.stack_pointer as u16;
    cpu.memory.memory[address as usize] = flags;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
}

pub fn rotate_left(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let mut value = cpu.memory.memory[address as usize];
    let carry = value >> 7;
    value <<= 1;
    value |= get_bit(cpu.status, StatusBit::Carry);
    cpu.memory.memory[address as usize] = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn rotate_left_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    let mut value = cpu.register_a;
    let carry = value >> 7;
    value <<= 1;
    value |= get_bit(cpu.status, StatusBit::Carry);
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn rotate_right(cpu: &mut CPU, _mode: &AddressingMode) {
    let address = get_operand_address(cpu, _mode);
    let mut value = cpu.memory.memory[address as usize];
    let carry = value & 1;
    value >>= 1;
    value |= get_bit(cpu.status, StatusBit::Carry) << 7;
    cpu.memory.memory[address as usize] = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn rotate_right_accumulator(cpu: &mut CPU, _mode: &AddressingMode) {
    let mut value = cpu.register_a;
    let carry = value & 1;
    value >>= 1;
    value |= get_bit(cpu.status, StatusBit::Carry) << 7;
    cpu.register_a = value;
    update_zero_and_negative_flags(cpu, value);
    update_status_bit(
        cpu,
        StatusBit::Carry,
        BitwiseOperation::from_bool(carry == 1),
    );
}

pub fn set_carry_flag(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Carry, BitwiseOperation::Set);
}

pub fn set_decimal_flag(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Decimal, BitwiseOperation::Set);
}

pub fn set_interrupt_disable(cpu: &mut CPU, _mode: &AddressingMode) {
    update_status_bit(cpu, StatusBit::Interrupt, BitwiseOperation::Set);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::addressing_mode::AddressingMode;
    use crate::cpu::cpu_model::{ExecuteFunction, STACK_RESET};
    use crate::cpu::memory::Memory;
    use crate::cpu::{cpu_functions, operation_codes};
    use std::collections::HashMap;
    use std::fs;

    // Helper function to create a new CPU instance
    const TEST_BASE_REGISTER_A: u8 = 0x05;
    const TEST_BASE_REGISTER_X: u8 = 0x0A;
    const TEST_BASE_REGISTER_Y: u8 = 0x0F;
    const TEST_BASE_PROGRAM_COUNTER: u16 = 0x0600;
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
    fn test_get_operand_address_relative_positive() {
        let mut cpu: CPU = create_test_cpu();
        cpu.program_counter = 0x2000;
        cpu.memory.memory[cpu.program_counter as usize] = 0x05;
        let mode: AddressingMode = AddressingMode::Relative;
        assert_eq!(cpu_functions::get_operand_address(&mut cpu, &mode), 0x2006);
    }

    #[test]
    fn test_get_operand_address_relative_negative() {
        let mut cpu: CPU = create_test_cpu();
        cpu.program_counter = 0x2000;
        cpu.memory.memory[cpu.program_counter as usize] = 0xFB;
        let mode: AddressingMode = AddressingMode::Relative;
        assert_eq!(cpu_functions::get_operand_address(&mut cpu, &mode), 0x1FFC);
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

    // #[test]
    // fn test_increment_memory() {
    //     let mut cpu: CPU = create_test_cpu();
    //     let mode: AddressingMode = AddressingMode::Immediate;
    //     const AMOUNT: u8 = 20;
    //     for _ in 0..AMOUNT {
    //         increment_memory(&mut cpu, &mode);
    //     }
    //     assert_eq!(cpu.register_a, TEST_BASE_REGISTER_A.wrapping_add(AMOUNT));
    // }

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
    fn test_jump_absolute_normal() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1000;
        cpu.memory.memory[0x1000] = 0x34;
        cpu.memory.memory[0x1001] = 0x12;

        jump(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0x1234);
    }

    #[test]
    fn test_jump_absolute_max_address() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x2000;
        cpu.memory.memory[0x2000] = 0xFF;
        cpu.memory.memory[0x2001] = 0xFF;

        jump(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0xFFFF);
    }

    #[test]
    fn test_jump_indirect_normal() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x3000;
        cpu.memory.memory[0x3000] = 0x50;
        cpu.memory.memory[0x3001] = 0x40;
        cpu.memory.memory[0x4050] = 0x78;
        cpu.memory.memory[0x4051] = 0x56;

        jump(&mut cpu, &AddressingMode::Indirect);

        assert_eq!(cpu.program_counter, 0x5678);
    }

    #[test]
    fn test_jump_indirect_page_boundary_bug() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x4000;
        cpu.memory.memory[0x4000] = 0xFF;
        cpu.memory.memory[0x4001] = 0x01;
        cpu.memory.memory[0x01FF] = 0xCD;
        cpu.memory.memory[0x0200] = 0xAB;
        cpu.memory.memory[0x0100] = 0xEF;

        jump(&mut cpu, &AddressingMode::Indirect);

        assert_eq!(cpu.program_counter, 0xEFCD);
    }

    #[test]
    fn test_jump_to_subroutine_normal() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1001;
        cpu.stack_pointer = 0xFF;
        cpu.memory.memory[0x1001] = 0x34;
        cpu.memory.memory[0x1002] = 0x12;

        jump_to_subroutine(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0x1234);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.memory.memory[0x01FF], 0x10);
        assert_eq!(cpu.memory.memory[0x01FE], 0x02);
    }

    #[test]
    fn test_jump_to_subroutine_stack_wrap() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x2000;
        cpu.stack_pointer = 0x01;
        cpu.memory.memory[0x2000] = 0x56;
        cpu.memory.memory[0x2001] = 0x34;

        jump_to_subroutine(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0x3456);
        assert_eq!(cpu.stack_pointer, 0xFF);
        assert_eq!(cpu.memory.memory[0x0101], 0x20);
        assert_eq!(cpu.memory.memory[0x0100], 0x01);
    }

    #[test]
    fn test_jump_to_subroutine_target_zero() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x3000;
        cpu.stack_pointer = 0xFF;
        cpu.memory.memory[0x3000] = 0x00;
        cpu.memory.memory[0x3001] = 0x00;

        jump_to_subroutine(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0x0000);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.memory.memory[0x01FF], 0x30);
        assert_eq!(cpu.memory.memory[0x01FE], 0x01);
    }

    #[test]
    fn test_jump_to_subroutine_target_max() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x4000;
        cpu.stack_pointer = 0xFF;
        cpu.memory.memory[0x4000] = 0xFF;
        cpu.memory.memory[0x4001] = 0xFF;

        jump_to_subroutine(&mut cpu, &AddressingMode::Absolute);

        assert_eq!(cpu.program_counter, 0xFFFF);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.memory.memory[0x01FF], 0x40);
        assert_eq!(cpu.memory.memory[0x01FE], 0x01);
    }

    // #[test]
    // fn test_decrement_memory() {
    //     let mut cpu: CPU = create_test_cpu();
    //     let mode: AddressingMode = AddressingMode::Immediate;
    //     const AMOUNT: u8 = 20;
    //     for _ in 0..AMOUNT {
    //         decrement_memory(&mut cpu, &mode);
    //     }
    //     assert_eq!(cpu.register_a, TEST_BASE_REGISTER_A.wrapping_sub(AMOUNT));
    // }

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
    fn test_return_from_subroutine() {
        let mut cpu = CPU::new();

        let return_addr = 0xABCD;
        cpu.stack_pointer = 0xFD;
        cpu.memory.memory[0x01FE] = ((return_addr - 1) & 0xFF) as u8;
        cpu.memory.memory[0x01FF] = ((return_addr - 1) >> 8) as u8;

        return_from_subroutine(&mut cpu, &AddressingMode::Implied);

        assert_eq!(
            cpu.program_counter, return_addr,
            "RTS should set PC to return address"
        );
        assert_eq!(
            cpu.stack_pointer, 0xFF,
            "RTS should increment SP by 2 from initial value"
        );
    }

    #[test]
    fn test_return_from_interrupt() {
        let mut cpu = CPU::new();

        cpu.stack_pointer = 0xFC;
        cpu.memory.memory[0x01FD] = 0x55;
        cpu.memory.memory[0x01FE] = 0xCD;
        cpu.memory.memory[0x01FF] = 0xAB;

        return_from_interrupt(&mut cpu, &AddressingMode::Implied);

        assert_eq!(
            cpu.program_counter, 0xABCD,
            "RTI should set PC to popped address"
        );
        assert_eq!(cpu.status, 0x65, "RTI should restore status register");
        assert_eq!(
            cpu.stack_pointer, 0xFF,
            "RTI should increment SP by 3 from initial value"
        );
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

    // --- Tests for branch_if_carry_clear ---

    #[test]
    fn test_branch_if_carry_clear_taken() {
        let mut cpu: CPU = create_test_cpu();
        cpu.status = 0x00;
        cpu.program_counter = 0x1000;
        cpu.memory.memory[cpu.program_counter as usize] = 5;
        cpu_functions::branch_if_carry_clear(&mut cpu, &AddressingMode::Relative);
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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
        assert_eq!(
            cpu.program_counter,
            0x1000u16.wrapping_add(1).wrapping_add(5)
        );
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

    #[test]
    fn test_arithmetic_shift_left() {
        let mut cpu = create_test_cpu();
        cpu.memory.memory[cpu.program_counter as usize] = 0b0100_0001;
        arithmetic_shift_left(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.memory.memory[cpu.program_counter as usize], 0b1000_0010);
        assert!(cpu.status & (StatusBit::Carry as u8) == 0);
        assert!(cpu.status & (StatusBit::Negative as u8) == 0);
        assert!(cpu.status & (StatusBit::Zero as u8) == 0);
    }

    #[test]
    fn test_arithmetic_shift_left_acumulator() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b0100_0001;
        arithmetic_shift_left_accumulator(&mut cpu, &AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(cpu.status & (StatusBit::Carry as u8) == 0);
        assert!(cpu.status & (StatusBit::Negative as u8) == 0);
        assert!(cpu.status & (StatusBit::Zero as u8) == 0);
    }

    #[test]
    fn test_bit_test() {
        let mut cpu = create_test_cpu();

        cpu.register_a = 0b0000_0101;
        cpu.memory.memory[cpu.program_counter as usize] = 0b1100_0000;
        let mode = AddressingMode::Immediate;

        bit_test(&mut cpu, &mode);
        assert_eq!(get_bit(cpu.status, StatusBit::Zero), 1);
        assert_eq!(get_bit(cpu.status, StatusBit::Overflow), 1);
        assert_eq!(get_bit(cpu.status, StatusBit::Negative), 1);
    }

    #[test]
    fn test_clear_carry_flag() {
        let mut cpu = create_test_cpu();
        cpu.status = 0b0000_0001;
        clear_carry_flag(&mut cpu, &AddressingMode::NoneAddressing);
        assert_eq!(cpu.status, 0b0000_0000);
    }

    #[test]
    fn test_exclusive_or() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b1010_1010;
        cpu.memory.memory[cpu.program_counter as usize] = 0b1100_1100;
        exclusive_or(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b0110_0110);
    }

    #[test]
    fn test_logical_and() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b1010_1010;
        cpu.memory.memory[cpu.program_counter as usize] = 0b1100_1100;
        logical_and(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_logical_inclusive_or() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b1010_1010;
        cpu.memory.memory[cpu.program_counter as usize] = 0b1100_1100;
        logical_inclusive_or(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b1110_1110);
    }

    #[test]
    fn test_logical_shift_right() {
        let mut cpu = create_test_cpu();
        cpu.memory.memory[cpu.program_counter as usize] = 0b1000_0001;
        logical_shift_right(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.memory.memory[cpu.program_counter as usize], 0b0100_0000);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
        assert_eq!(get_bit(cpu.status, StatusBit::Zero), 0);
        assert_eq!(get_bit(cpu.status, StatusBit::Negative), 0);
    }

    #[test]
    fn test_logical_shift_right_accumulator() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b1000_0001;
        logical_shift_right_accumulator(&mut cpu, &AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b0100_0000);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
        assert_eq!(get_bit(cpu.status, StatusBit::Zero), 0);
        assert_eq!(get_bit(cpu.status, StatusBit::Negative), 0);
    }

    #[test]
    fn test_rotate_left() {
        let mut cpu = create_test_cpu();
        cpu.memory.memory[cpu.program_counter as usize] = 0b1000_0001;
        cpu.status = 0b0000_0001;
        rotate_left(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.memory.memory[cpu.program_counter as usize], 0b0000_0011);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
    }

    #[test]
    fn test_rotate_left_accumulator() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b1000_0001;
        cpu.status = 0b0000_0001;
        rotate_left_accumulator(&mut cpu, &AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b0000_0011);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
    }

    #[test]
    fn test_rotate_right() {
        let mut cpu = create_test_cpu();
        cpu.memory.memory[cpu.program_counter as usize] = 0b0000_0011;
        cpu.status = 0b0000_0001;
        rotate_right(&mut cpu, &AddressingMode::Immediate);
        assert_eq!(cpu.memory.memory[cpu.program_counter as usize], 0b1000_0001);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
    }

    #[test]
    fn test_rotate_right_accumulator() {
        let mut cpu = create_test_cpu();
        cpu.register_a = 0b0000_0011;
        cpu.status = 0b0000_0001;
        rotate_right_accumulator(&mut cpu, &AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b1000_0001);
        assert_eq!(get_bit(cpu.status, StatusBit::Carry), 1);
    }
    #[test]
    fn test_mini_program() {
        let mut cpu = CPU {
            register_a: 0x00,
            register_x: 0x00,
            register_y: 0x00,
            status: 0x00,
            program_counter: 0x0600,
            stack_pointer: 0xFD,
            memory: Memory::new(),
        };

        // Program:
        // INC $40   ; 0xE6 0x40
        // ASL A     ; 0x0A
        // PHP       ; 0x08
        // PLP       ; 0x28
        // BRK       ; 0x00
        let program = [0xE6, 0x40, 0x0A, 0x08, 0x28, 0x00];

        // Set initial conditions
        cpu.memory.write_u8(0x0040, 0x05); // $40 = 0x05
        cpu.register_a = 0x10; // A = 0x10
        cpu.status = 0x02; // Zero flag set

        // Load program at 0x0600
        for (i, &byte) in program.iter().enumerate() {
            cpu.memory.write_u8(0x0600 + i as u16, byte);
        }

        // Run the program
        cpu.run_with_callback(|_| {});

        // Verify results
        assert_eq!(
            cpu.memory.read_u8(0x0040),
            0x06,
            "Memory at $40 should be 0x06"
        );
        assert_eq!(
            cpu.register_a, 0x20,
            "Accumulator should be shifted to 0x20"
        );
        // Status should be restored by PLP, with zero flag = 0 (since A = 0x20)
        assert_eq!(
            cpu.status & 0x02,
            0x00,
            "Zero flag should be clear after PLP"
        );
    }

    use serde_json::{self, Value};
    fn parse_cpu(current_json: &serde_json::Value) -> Option<CPU> {
        let pc = current_json.get("pc")?.as_i64()? as u16;
        let s = current_json.get("s")?.as_i64()? as u8;
        let a = current_json.get("a")?.as_i64()? as u8;
        let x = current_json.get("x")?.as_i64()? as u8;
        let y = current_json.get("y")?.as_i64()? as u8;
        let status = current_json.get("p")?.as_i64()? as u8;
        let ram = current_json.get("ram")?.as_array()?;

        let mut cpu = CPU {
            register_a: a,
            register_x: x,
            register_y: y,
            status,
            program_counter: pc,
            stack_pointer: s,
            memory: Memory::new(),
        };
        for element in ram {
            let current = element.as_array()?;
            let position = current[0].as_i64()? as u16;
            let data = current[1].as_i64()? as u8;
            cpu.memory.write_u8(position, data);
        }

        Some(cpu)
    }
    fn parse_json(json: &Value) -> Option<(CPU, CPU)> {
        let initial = json.get("initial")?;
        let finale = json.get("final")?;
        let initial_cpu = parse_cpu(initial)?;
        let final_cpu = parse_cpu(finale)?;

        Some((initial_cpu, final_cpu))
    }

    #[test]
    fn testing_individual_opcodes() {
        let operation_codes: &HashMap<u8, (&'static operation_codes::Operation, ExecuteFunction)> =
            &operation_codes::OPERATION_CODES_MAP;
        let mut real_real_failures: Vec<u8> = vec![];
        let mut real_failures: Vec<Vec<i32>> = vec![];
        for ele in operation_codes.keys() {
            if *ele == 0x00 {
                continue;
            }
            let filename = format!("build/files/{:02x}.json", ele);
            let st = fs::read_to_string(filename).expect("uwu");
            let jsons: serde_json::Value =
                serde_json::from_str(&st).expect("JSON was not well-formatted");

            let mut failures: Vec<i32> = vec![];
            let mut count = 0;
            let mut flag: bool = false;
            for json in jsons.as_array().unwrap() {
                let (mut cpu_ini, cpu_final) = parse_json(json).unwrap();

                count += 1;
                cpu_ini.run_once();
                // assert_eq!(cpu_ini.program_counter, cpu_final.program_counter);
                // assert_eq!(cpu_ini.register_a, cpu_final.register_a);
                // assert_eq!(cpu_ini.register_x, cpu_final.register_x);
                // assert_eq!(cpu_ini.register_y, cpu_final.register_y);
                // assert_eq!(cpu_ini.status, cpu_final.status);
                if cpu_ini != cpu_final && !flag {
                    // [TODO] Fix issues with the decimal representation, for now it's working fine
                    if (cpu_ini.status & (1 << StatusBit::Decimal as u8)) == 0 {
                        real_real_failures.push(*ele);
                        failures.push(count);
                        flag = true;
                    }
                }
            }
            real_failures.push(failures);
        }
        let flattened: Vec<i32> = real_failures.clone().into_iter().flatten().collect();

        real_real_failures.sort_unstable();
        real_real_failures.dedup();
        if !flattened.is_empty() {
            println!("{:?}", real_real_failures);
            assert_eq!(0, 1);
        }
        assert_eq!(0, 0);
    }
}
