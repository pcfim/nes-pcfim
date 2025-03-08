use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::cpu_functions;
use crate::cpu::cpu_model::ExecuteFunction;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum OperationName {
    AddWithCarry,
    ArithmeticShiftLeft,
    BitTest,
    BranchIfCarryClear,
    BranchIfCarrySet,
    BranchIfEqual,
    BranchIfMinus,
    BranchIfNotEqual,
    BranchIfOverflowClear,
    BranchIfOverflowSet,
    BranchIfPositive,
    ClearCarryFlag,
    ClearDecimalMode,
    ClearInterruptDisable,
    ClearOverflowFlag,
    Compare,
    CompareX,
    CompareY,
    DecrementMemory,
    DecrementXRegister,
    DecrementYRegister,
    ExclusiveOR,
    ForceInterrupt,
    IncrementMemory,
    IncrementXRegister,
    IncrementYRegister,
    Jump,
    JumpToSubroutine,
    LoadAccumulator,
    LoadXRegister,
    LoadYRegister,
    LogicalAND,
    LogicalInclusiveOR,
    LogicalShiftRight,
    PullAccumulator,
    PullProcessorStatus,
    PushAccumulator,
    PushProcessorStatus,
    ReturnFromInterrupt,
    ReturnFromSubroutine,
    RotateLeft,
    RotateRight,
    SetCarryFlag,
    SetDecimalFlag,
    SetInterruptDisable,
    StoreAccumulator,
    StoreXRegister,
    StoreYRegister,
    SubstractWithCarry,
    TransferAccumulatorToX,
    TransferAccumulatorToY,
    TransferStackPointerToX,
    TransferXToAccumulator,
    TransferXToStackPointer,
    TransferYToAccumulator,
}

pub struct Operation {
    pub operation_code: u8,
    pub len: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}
impl Operation {
    fn new(operation_code: u8, len: u8, cycles: u8, addressing_mode: AddressingMode) -> Self {
        Operation {
            operation_code,
            len,
            cycles,
            addressing_mode,
        }
    }
}
pub struct OperationCodes {
    pub operation_name: OperationName,
    pub operations: Vec<Operation>,
    pub execute_function: ExecuteFunction,
}
impl OperationCodes {
    fn new(
        operation_name: OperationName,
        operations: Vec<Operation>,
        execute_function: ExecuteFunction,
    ) -> Self {
        OperationCodes {
            operation_name,
            operations,
            execute_function,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OperationCodes> = vec![
        OperationCodes::new(
            OperationName::BranchIfCarryClear,
            vec![Operation::new(0x90, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_carry_clear
        ),
        OperationCodes::new(
            OperationName::BranchIfCarrySet,
            vec![Operation::new(0xb0, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_carry_set
        ),
        OperationCodes::new(
            OperationName::BranchIfEqual,
            vec![Operation::new(0xf0, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_equal
        ),
        OperationCodes::new(
            OperationName::BranchIfMinus,
            vec![Operation::new(0x30, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_minus
        ),
        OperationCodes::new(
            OperationName::BranchIfNotEqual,
            vec![Operation::new(0xd0, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_not_equal
        ),
        OperationCodes::new(
            OperationName::BranchIfPositive,
            vec![Operation::new(0x10, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_positive
        ),
        OperationCodes::new(
            OperationName::BranchIfOverflowClear,
            vec![Operation::new(0x50, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_overflow_clear
        ),
        OperationCodes::new(
            OperationName::BranchIfOverflowSet,
            vec![Operation::new(0x70, 2, 2, AddressingMode::Relative),],
            cpu_functions::branch_if_overflow_set
        ),
        OperationCodes::new(
            OperationName::ForceInterrupt,
            vec![Operation::new(0x00, 1, 7, AddressingMode::NoneAddressing),],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::TransferAccumulatorToX,
            vec![Operation::new(0xaa, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::transfer_accumulator_to_x
        ),
        OperationCodes::new(
            OperationName::TransferAccumulatorToY,
            vec![Operation::new(0xaa, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::transfer_accumulator_to_y
        ),
        OperationCodes::new(
            OperationName::IncrementXRegister,
            vec![Operation::new(0xe8, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::increment_x_register
        ),
        OperationCodes::new(
            OperationName::IncrementYRegister,
            vec![Operation::new(0xc8, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::increment_y_register
        ),
        OperationCodes::new(
            OperationName::IncrementMemory,
            vec![
                Operation::new(0xe6, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0xf6, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0xee, 3, 6, AddressingMode::Absolute),
                Operation::new(0xfe, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::increment_memory
        ),
        OperationCodes::new(
            OperationName::Jump,
            vec![
                Operation::new(0x4c, 3, 3, AddressingMode::Absolute),
                Operation::new(0x6c, 3, 5, AddressingMode::Indirect),
            ],
            cpu_functions::jump
        ),
        OperationCodes::new(
            OperationName::JumpToSubroutine,
            vec![Operation::new(0x20, 3, 6, AddressingMode::Absolute),],
            cpu_functions::jump_to_subroutine
        ),
        OperationCodes::new(
            OperationName::DecrementXRegister,
            vec![Operation::new(0xca, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::decrement_x_register
        ),
        OperationCodes::new(
            OperationName::DecrementYRegister,
            vec![Operation::new(0x88, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::decrement_y_register
        ),
        OperationCodes::new(
            OperationName::DecrementMemory,
            vec![
                Operation::new(0xc6, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0xd6, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0xce, 3, 6, AddressingMode::Absolute),
                Operation::new(0xde, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::decrement_memory
        ),
        OperationCodes::new(
            OperationName::LoadAccumulator,
            vec![
                Operation::new(0xa9, 2, 2, AddressingMode::Immediate),
                Operation::new(0xa5, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xb5, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0xad, 3, 4, AddressingMode::Absolute),
                Operation::new(0xbd, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0xb9, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0xa1, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0xb1, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::load_accumulator
        ),
        OperationCodes::new(
            OperationName::LoadXRegister,
            vec![
                Operation::new(0xA2, 2, 2, AddressingMode::Immediate),
                Operation::new(0xA6, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xB6, 2, 4, AddressingMode::ZeroPage_Y),
                Operation::new(0xAE, 3, 4, AddressingMode::Absolute),
                Operation::new(0xBE, 3, 4, AddressingMode::Absolute_Y),
            ],
            cpu_functions::load_x_register
        ),
        OperationCodes::new(
            OperationName::LoadYRegister,
            vec![
                Operation::new(0xA0, 2, 2, AddressingMode::Immediate),
                Operation::new(0xA4, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xB4, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0xAC, 3, 4, AddressingMode::Absolute),
                Operation::new(0xBC, 3, 4, AddressingMode::Absolute_X),
            ],
            cpu_functions::load_y_register
        ),
        OperationCodes::new(
            OperationName::ReturnFromInterrupt,
            vec![Operation::new(0x40, 1, 6, AddressingMode::Implied),],
            cpu_functions::return_from_interrupt
        ),
        OperationCodes::new(
            OperationName::ReturnFromSubroutine,
            vec![Operation::new(0x60, 1, 6, AddressingMode::Implied),],
            cpu_functions::return_from_subroutine
        ),
        OperationCodes::new(
            OperationName::StoreAccumulator,
            vec![
                Operation::new(0x85, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x95, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x8d, 3, 4, AddressingMode::Absolute),
                Operation::new(0x9d, 3, 5, AddressingMode::Absolute_X),
                Operation::new(0x99, 3, 5, AddressingMode::Absolute_Y),
                Operation::new(0x81, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0x91, 2, 6, AddressingMode::Indirect_Y),
            ],
            cpu_functions::store_accumulator
        ),
        OperationCodes::new(
            OperationName::StoreXRegister,
            vec![
                Operation::new(0x86, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x96, 2, 4, AddressingMode::ZeroPage_Y),
                Operation::new(0x8e, 3, 4, AddressingMode::Absolute),
            ],
            cpu_functions::store_x_register
        ),
        OperationCodes::new(
            OperationName::StoreYRegister,
            vec![
                Operation::new(0x84, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x94, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x8c, 3, 4, AddressingMode::Absolute),
            ],
            cpu_functions::store_y_register
        ),
        OperationCodes::new(
            OperationName::Compare,
            vec![
                Operation::new(0xC9, 2, 2, AddressingMode::Immediate),
                Operation::new(0xC5, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xD5, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0xCD, 3, 4, AddressingMode::Absolute),
                Operation::new(0xDD, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0xD9, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0xC1, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0xD1, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::compare_a
        ),
        OperationCodes::new(
            OperationName::CompareX,
            vec![
                Operation::new(0xE0, 2, 2, AddressingMode::Immediate),
                Operation::new(0xE4, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xEC, 3, 4, AddressingMode::Absolute)
            ],
            cpu_functions::compare_x
        ),
        OperationCodes::new(
            OperationName::CompareY,
            vec![
                Operation::new(0xC0, 2, 2, AddressingMode::Immediate),
                Operation::new(0xC4, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xCC, 3, 4, AddressingMode::Absolute)
            ],
            cpu_functions::compare_y
        ),
        OperationCodes::new(
            OperationName::AddWithCarry,
            vec![
                Operation::new(0x69, 2, 2, AddressingMode::Immediate),
                Operation::new(0x65, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x75, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x6d, 3, 4, AddressingMode::Absolute),
                Operation::new(0x7d, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0x79, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0x61, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0x71, 2, 5, AddressingMode::Indirect_Y)
            ],
            cpu_functions::add_with_carry
        ),
        OperationCodes::new(
            OperationName::SubstractWithCarry,
            vec![
                Operation::new(0xe9, 2, 2, AddressingMode::Immediate),
                Operation::new(0xe5, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xf5, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0xed, 3, 4, AddressingMode::Absolute),
                Operation::new(0xfd, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0xf9, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0xe1, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0xf1, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::substract_with_carry
        ),
        OperationCodes::new(
            OperationName::TransferStackPointerToX,
            vec![Operation::new(0xba, 1, 2, AddressingMode::NoneAddressing),],
            cpu_functions::transfer_stack_pointer_to_x
        ),
        OperationCodes::new(
            OperationName::TransferXToAccumulator,
            vec![Operation::new(0x8a, 1, 2, AddressingMode::NoneAddressing),],
            cpu_functions::transfer_x_to_accumulator
        ),
        OperationCodes::new(
            OperationName::TransferXToStackPointer,
            vec![Operation::new(0x9a, 1, 2, AddressingMode::NoneAddressing),],
            cpu_functions::transfer_x_to_stack_pointer
        ),
        OperationCodes::new(
            OperationName::TransferYToAccumulator,
            vec![Operation::new(0x98, 1, 2, AddressingMode::NoneAddressing),],
            cpu_functions::transfer_y_to_accumulator
        ),
        OperationCodes::new(
            OperationName::ArithmeticShiftLeft,
            vec![
                Operation::new(0x0a, 1, 2, AddressingMode::Accumulator),
                Operation::new(0x06, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0x16, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0x0e, 3, 6, AddressingMode::Absolute),
                Operation::new(0x1e, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::arithmetic_shift_left
        ),
        OperationCodes::new(
            OperationName::BitTest,
            vec![
                Operation::new(0x24, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x2c, 3, 4, AddressingMode::Absolute),
            ],
            cpu_functions::bit_test
        ),
        OperationCodes::new(
            OperationName::ClearCarryFlag,
            vec![Operation::new(0x18, 1, 2, AddressingMode::Implied),],
            cpu_functions::clear_carry_flag
        ),
        OperationCodes::new(
            OperationName::ClearDecimalMode,
            vec![Operation::new(0xd8, 1, 2, AddressingMode::Implied),],
            cpu_functions::clear_decimal_mode
        ),
        OperationCodes::new(
            OperationName::ClearInterruptDisable,
            vec![Operation::new(0x58, 1, 2, AddressingMode::Implied),],
            cpu_functions::clear_interrupt_disable
        ),
        OperationCodes::new(
            OperationName::ClearOverflowFlag,
            vec![Operation::new(0xb8, 1, 2, AddressingMode::Implied),],
            cpu_functions::clear_overflow_flag
        ),
        OperationCodes::new(
            OperationName::ExclusiveOR,
            vec![
                Operation::new(0x49, 2, 2, AddressingMode::Immediate),
                Operation::new(0x45, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x55, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x4d, 3, 4, AddressingMode::Absolute),
                Operation::new(0x5d, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0x59, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0x41, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0x51, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::exclusive_or
        ),
        OperationCodes::new(
            OperationName::LogicalAND,
            vec![
                Operation::new(0x29, 2, 2, AddressingMode::Immediate),
                Operation::new(0x25, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x35, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x2d, 3, 4, AddressingMode::Absolute),
                Operation::new(0x3d, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0x39, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0x21, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0x31, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::logical_and
        ),
        OperationCodes::new(
            OperationName::LogicalInclusiveOR,
            vec![
                Operation::new(0x09, 2, 2, AddressingMode::Immediate),
                Operation::new(0x05, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x15, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x0d, 3, 4, AddressingMode::Absolute),
                Operation::new(0x1d, 3, 4, AddressingMode::Absolute_X),
                Operation::new(0x19, 3, 4, AddressingMode::Absolute_Y),
                Operation::new(0x01, 2, 6, AddressingMode::Indirect_X),
                Operation::new(0x11, 2, 5, AddressingMode::Indirect_Y),
            ],
            cpu_functions::logical_inclusive_or
        ),
        OperationCodes::new(
            OperationName::LogicalShiftRight,
            vec![
                Operation::new(0x4a, 1, 2, AddressingMode::Accumulator),
                Operation::new(0x46, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0x56, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0x4e, 3, 6, AddressingMode::Absolute),
                Operation::new(0x5e, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::logical_shift_right
        ),
        OperationCodes::new(
            OperationName::PullAccumulator,
            vec![Operation::new(0x68, 1, 4, AddressingMode::Implied),],
            cpu_functions::pull_accumulator
        ),
        OperationCodes::new(
            OperationName::PullProcessorStatus,
            vec![Operation::new(0x28, 1, 4, AddressingMode::Implied),],
            cpu_functions::pull_processor_status
        ),
        OperationCodes::new(
            OperationName::PushAccumulator,
            vec![Operation::new(0x48, 1, 3, AddressingMode::Implied),],
            cpu_functions::push_accumulator
        ),
        OperationCodes::new(
            OperationName::PushProcessorStatus,
            vec![Operation::new(0x08, 1, 3, AddressingMode::Implied),],
            cpu_functions::push_processor_status
        ),
        OperationCodes::new(
            OperationName::RotateLeft,
            vec![
                Operation::new(0x2a, 1, 2, AddressingMode::Accumulator),
                Operation::new(0x26, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0x36, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0x2e, 3, 6, AddressingMode::Absolute),
                Operation::new(0x3e, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::rotate_left
        ),
        OperationCodes::new(
            OperationName::RotateRight,
            vec![
                Operation::new(0x6a, 1, 2, AddressingMode::Accumulator),
                Operation::new(0x66, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0x76, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0x6e, 3, 6, AddressingMode::Absolute),
                Operation::new(0x7e, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::rotate_right
        ),
        OperationCodes::new(
            OperationName::SetCarryFlag,
            vec![Operation::new(0x38, 1, 2, AddressingMode::Implied),],
            cpu_functions::set_carry_flag
        ),
        OperationCodes::new(
            OperationName::SetDecimalFlag,
            vec![Operation::new(0xf8, 1, 2, AddressingMode::Implied),],
            cpu_functions::set_decimal_flag
        ),
        OperationCodes::new(
            OperationName::SetInterruptDisable,
            vec![Operation::new(0x78, 1, 2, AddressingMode::Implied),],
            cpu_functions::set_interrupt_disable
        )
    ];
    pub static ref OPERATION_CODES_MAP: HashMap<u8, (&'static Operation, ExecuteFunction)> = {
        let mut map = HashMap::new();
        for cpu_operation in &*CPU_OPS_CODES {
            for cpu_op in &*cpu_operation.operations {
                map.insert(
                    cpu_op.operation_code,
                    (cpu_op, cpu_operation.execute_function),
                );
            }
        }
        map
    };
}
