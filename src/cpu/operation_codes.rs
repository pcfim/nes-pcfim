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

pub struct OperationCodes {
    pub operation_code: u8,
    pub operation_name: OperationName,
    pub len: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl OperationCodes {
    fn new(
        operation_code: u8,
        operation_name: OperationName,
        len: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        OperationCodes {
            operation_code,
            operation_name,
            len,
            cycles,
            addressing_mode,
        }
    }
}

lazy_static! {
  pub static ref CPU_OPS_CODES: Vec<OperationCodes> = vec![
      OperationCodes::new(0x00, OperationName::ForceInterrupt, 1, 7, AddressingMode::NoneAddressing),
      OperationCodes::new(0xaa, OperationName::TransferAccumulatorToX, 1, 2, AddressingMode::NoneAddressing),
      OperationCodes::new(0xe8, OperationName::IncrementXRegister, 1, 2, AddressingMode::NoneAddressing),

      OperationCodes::new(0xa9, OperationName::LoadAccumulator, 2, 2, AddressingMode::Immediate),
      OperationCodes::new(0xa5, OperationName::LoadAccumulator, 2, 3, AddressingMode::ZeroPage),
      OperationCodes::new(0xb5, OperationName::LoadAccumulator, 2, 4, AddressingMode::ZeroPage_X),
      OperationCodes::new(0xad, OperationName::LoadAccumulator, 3, 4, AddressingMode::Absolute),
      OperationCodes::new(0xbd, OperationName::LoadAccumulator, 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),
      OperationCodes::new(0xb9, OperationName::LoadAccumulator, 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),
      OperationCodes::new(0xa1, OperationName::LoadAccumulator, 2, 6, AddressingMode::Indirect_X),
      OperationCodes::new(0xb1, OperationName::LoadAccumulator, 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),

      OperationCodes::new(0x85, OperationName::StoreAccumulator, 2, 3, AddressingMode::ZeroPage),
      OperationCodes::new(0x95, OperationName::StoreAccumulator, 2, 4, AddressingMode::ZeroPage_X),
      OperationCodes::new(0x8d, OperationName::StoreAccumulator, 3, 4, AddressingMode::Absolute),
      OperationCodes::new(0x9d, OperationName::StoreAccumulator, 3, 5, AddressingMode::Absolute_X),
      OperationCodes::new(0x99, OperationName::StoreAccumulator, 3, 5, AddressingMode::Absolute_Y),
      OperationCodes::new(0x81, OperationName::StoreAccumulator, 2, 6, AddressingMode::Indirect_X),
      OperationCodes::new(0x91, OperationName::StoreAccumulator, 2, 6, AddressingMode::Indirect_Y),

  ];


  pub static ref OPERATION_CODES_MAP: HashMap<u8, &'static OperationCodes> = {
      let mut map = HashMap::new();
      for cpuop in &*CPU_OPS_CODES {
          map.insert(cpuop.operation_code, cpuop);
      }
      map
  };
}
