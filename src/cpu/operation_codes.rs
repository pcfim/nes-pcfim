use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::cpu_functions;
use crate::cpu::cpu_model::ExecuteFunction;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum OperationName {
    BranchIfCarryClear,
    BranchIfCarrySet,
    BranchIfEqual,
    BranchIfMinus,
    BranchIfNotEqual,
    BranchIfPositive,
    BranchIfOverflowClear,
    BranchIfOverflowSet,
    ForceInterrupt,
    TransferAccumulatorToX,
    LoadAccumulator,
    StoreAccumulator,
    IncrementXRegister,
    CompareX,
    CompareY,
    Compare,
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
            OperationName::IncrementXRegister,
            vec![Operation::new(0xe8, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::increment_x_register
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
