use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    //invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

}