use crate::{ExitError, Opcode};
use primitive_types::{H160, H256, U256};
use sha3::{Digest, Keccak256};

/// Runtime state.
#[derive(Clone, Debug)]
pub struct RuntimeState {
	/// Runtime context.
	pub context: Context,
	/// Return data buffer.
	pub retbuf: Vec<u8>,
	/// Current gas.
	pub gas: U256,
}

impl AsRef<RuntimeState> for RuntimeState {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl AsMut<RuntimeState> for RuntimeState {
	fn as_mut(&mut self) -> &mut Self {
		self
	}
}

/// Context of the runtime.
#[derive(Clone, Debug)]
pub struct Context {
	/// Execution address.
	pub address: H160,
	/// Caller of the EVM.
	pub caller: H160,
	/// Apparent value of the EVM.
	pub apparent_value: U256,
}

/// Transfer from source to target, with given value.
#[derive(Clone, Debug)]
pub struct Transfer {
	/// Source address.
	pub source: H160,
	/// Target address.
	pub target: H160,
	/// Transfer value.
	pub value: U256,
}

/// Log
#[derive(Clone, Debug)]
pub struct Log {
	pub address: H160,
	pub topics: Vec<H256>,
	pub data: Vec<u8>,
}

pub trait CallCreateTrap: Sized {
	fn call_create_trap(opcode: Opcode) -> Self;
}

impl CallCreateTrap for Opcode {
	fn call_create_trap(opcode: Opcode) -> Self {
		opcode
	}
}

pub trait RuntimeBaseBackend {
	/// Get environmental block hash.
	fn block_hash(&self, number: U256) -> H256;
	/// Get environmental block number.
	fn block_number(&self) -> U256;
	/// Get environmental coinbase.
	fn block_coinbase(&self) -> H160;
	/// Get environmental block timestamp.
	fn block_timestamp(&self) -> U256;
	/// Get environmental block difficulty.
	fn block_difficulty(&self) -> U256;
	/// Get environmental block randomness.
	fn block_randomness(&self) -> Option<H256>;
	/// Get environmental gas limit.
	fn block_gas_limit(&self) -> U256;
	/// Environmental block base fee.
	fn block_base_fee_per_gas(&self) -> U256;
	/// Get environmental chain ID.
	fn chain_id(&self) -> U256;
	/// Get the gas price value.
	fn gas_price(&self) -> U256;
	/// Get execution origin.
	fn origin(&self) -> H160;

	/// Get balance of address.
	fn balance(&self, address: H160) -> U256;
	/// Get code size of address.
	fn code_size(&self, address: H160) -> U256 {
		U256::from(self.code(address).len())
	}
	/// Get code hash of address.
	fn code_hash(&self, address: H160) -> H256 {
		H256::from_slice(&Keccak256::digest(&self.code(address)[..]))
	}
	/// Get code of address.
	fn code(&self, address: H160) -> Vec<u8>;
	/// Get storage value of address at index.
	fn storage(&self, address: H160, index: H256) -> H256;

	/// Check whether an address exists.
	fn exists(&self, address: H160) -> bool;

	/// Get the current nonce of an account.
	fn nonce(&self, address: H160) -> U256;
}

/// The distinguish between `RuntimeBaseBackend` and `RuntimeBackend` is for the implementation of
/// overlays.
pub trait RuntimeBackend: RuntimeBaseBackend {
	/// Get original storage value of address at index.
	fn original_storage(&self, address: H160, index: H256) -> H256;
	/// Check whether an address has already been deleted.
	fn deleted(&self, address: H160) -> bool;
	/// Checks if the address or (address, index) pair has been previously accessed.
	fn is_cold(&self, address: H160, index: Option<H256>) -> bool;
	/// Mark an address or (address, index) pair as hot.
	fn mark_hot(&mut self, address: H160, index: Option<H256>) -> Result<(), ExitError>;
	/// Set storage value of address at index.
	fn set_storage(&mut self, address: H160, index: H256, value: H256) -> Result<(), ExitError>;
	/// Create a log owned by address with given topics and data.
	fn log(&mut self, log: Log) -> Result<(), ExitError>;
	/// Mark an address to be deleted, with funds transferred to target.
	fn mark_delete(&mut self, address: H160, target: H160) -> Result<(), ExitError>;
	/// Fully delete storages of an account.
	fn reset_storage(&mut self, address: H160);
	/// Set code of an account.
	fn set_code(&mut self, address: H160, code: Vec<u8>);
	/// Reset balance of an account.
	fn reset_balance(&mut self, address: H160);
	/// Initiate a transfer.
	fn transfer(&mut self, transfer: Transfer) -> Result<(), ExitError>;
	/// Increase the nonce value.
	fn inc_nonce(&mut self, address: H160) -> Result<(), ExitError>;
}
