pub mod error;

mod addressing_modes;
use addressing_modes::AddressingMode;
mod opcodes;
use opcodes::OpcodeType;
mod emulator;
pub use emulator::Emulator;
