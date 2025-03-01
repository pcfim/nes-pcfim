use crate::cpu::addressing_mode::AddressingMode;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum OperationName {
    ForceInterrupt,
    TransferAccumulatorToX,
    LoadAccumulator,
    StoreAccumulator,
    IncrementXRegister,
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
}
impl OperationCodes {
    fn new(operation_name: OperationName, operations: Vec<Operation>) -> Self {
        OperationCodes {
            operation_name,
            operations,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OperationCodes> = vec![
        OperationCodes::new(
            OperationName::ForceInterrupt,
            vec![Operation::new(0x00, 1, 7, AddressingMode::NoneAddressing),]
        ),
        OperationCodes::new(
            OperationName::TransferAccumulatorToX,
            vec![Operation::new(0xaa, 1, 2, AddressingMode::NoneAddressing)]
        ),
        OperationCodes::new(
            OperationName::IncrementXRegister,
            vec![Operation::new(0xe8, 1, 2, AddressingMode::NoneAddressing)]
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
            ]
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
            ]
        )
    ];
    pub static ref OPERATION_CODES_MAP: HashMap<u8, &'static Operation> = {
        let mut map = HashMap::new();
        for cpu_operation in &*CPU_OPS_CODES {
            for cpu_op in &*cpu_operation.operations {
                map.insert(cpu_op.operation_code, cpu_op);
            }
        }
        map
    };
}
