use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::cpu_functions;
use crate::cpu::cpu_model::ExecuteFunction;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum OperationName {
    ForceInterrupt,
    TransferAccumulatorToX,
    TransferAccumulatorToY,
    LoadAccumulator,
    LoadAccumulatorX,
    LoadAccumulatorY,
    StoreAccumulator,
    StoreAccumulatorX,
    StoreAccumulatorY,
    IncrementMemory,
    IncrementXRegister,
    IncrementYRegister,
    DecrementMemory,
    DecrementXRegister,
    DecrementYRegister,
    CompareX,
    CompareY,
    Compare,
    AddWithCarry,
    SubstractWithCarry,
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
            OperationName::ForceInterrupt,
            vec![Operation::new(0x00, 1, 7, AddressingMode::NoneAddressing),],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::TransferAccumulatorToX,
            vec![Operation::new(0xaa, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::TransferAccumulatorToY,
            vec![Operation::new(0xaa, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::IncrementXRegister,
            vec![Operation::new(0xe8, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::increment_x_register
        ),
        OperationCodes::new(
            OperationName::IncrementYRegister,
            vec![Operation::new(0xc8, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::IncrementMemory,
            vec![
                Operation::new(0xe6, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0xf6, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0xee, 3, 6, AddressingMode::Absolute),
                Operation::new(0xfe, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::DecrementXRegister,
            vec![Operation::new(0xca, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::increment_x_register
        ),
        OperationCodes::new(
            OperationName::DecrementYRegister,
            vec![Operation::new(0x88, 1, 2, AddressingMode::NoneAddressing)],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::DecrementMemory,
            vec![
                Operation::new(0xc6, 2, 5, AddressingMode::ZeroPage),
                Operation::new(0xd6, 2, 6, AddressingMode::ZeroPage_X),
                Operation::new(0xce, 3, 6, AddressingMode::Absolute),
                Operation::new(0xde, 3, 7, AddressingMode::Absolute_X),
            ],
            cpu_functions::force_interruptions
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
            cpu_functions::load_accumulator_a
        ),
        OperationCodes::new(
            OperationName::LoadAccumulatorX,
            vec![
                Operation::new(0xA2, 2, 2, AddressingMode::Immediate),
                Operation::new(0xA6, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xB6, 2, 4, AddressingMode::ZeroPage_Y),
                Operation::new(0xAE, 3, 4, AddressingMode::Absolute),
                Operation::new(0xBE, 3, 4, AddressingMode::Absolute_Y),
            ],
            cpu_functions::load_accumulator_x
        ),
        OperationCodes::new(
            OperationName::LoadAccumulatorY,
            vec![
                Operation::new(0xA0, 2, 2, AddressingMode::Immediate),
                Operation::new(0xA4, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0xB4, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0xAC, 3, 4, AddressingMode::Absolute),
                Operation::new(0xBC, 3, 4, AddressingMode::Absolute_X),
            ],
            cpu_functions::load_accumulator_y
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
            OperationName::StoreAccumulatorX,
            vec![
                Operation::new(0x86, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x96, 2, 4, AddressingMode::ZeroPage_Y),
                Operation::new(0x8e, 3, 4, AddressingMode::Absolute),
            ],
            cpu_functions::force_interruptions
        ),
        OperationCodes::new(
            OperationName::StoreAccumulatorY,
            vec![
                Operation::new(0x84, 2, 3, AddressingMode::ZeroPage),
                Operation::new(0x94, 2, 4, AddressingMode::ZeroPage_X),
                Operation::new(0x8c, 3, 4, AddressingMode::Absolute),
            ],
            cpu_functions::force_interruptions
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
            cpu_functions::force_interruptions
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
            cpu_functions::force_interruptions
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
            cpu_functions::force_interruptions
        ),
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
