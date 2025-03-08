#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum BitwiseOperation {
    Set,
    Unset,
    Flip,
}

impl BitwiseOperation {
    pub fn from_bool(value: bool) -> Self {
        if value {
            BitwiseOperation::Set
        } else {
            BitwiseOperation::Unset
        }
    }
}
