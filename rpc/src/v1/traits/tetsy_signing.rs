// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Tetsy Vapory.

// Tetsy Vapory is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy Vapory is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy Vapory.  If not, see <http://www.gnu.org/licenses/>.

//! TetsySigning rpc interface.
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

use vapory_types::{H160, U256};
use v1::types::{Bytes, ConfirmationResponse, TransactionRequest, Either};

/// Signing methods implementation.
#[rpc(server)]
pub trait TetsySigning {
	/// RPC Metadata
	type Metadata;

	/// Given partial transaction request produces transaction with all fields filled in.
	/// Such transaction can be then signed externally.
	#[rpc(meta, name = "tetsy_composeTransaction")]
	fn compose_transaction(&self, _: Self::Metadata, _: TransactionRequest) -> BoxFuture<TransactionRequest>;

	/// Posts sign request asynchronously.
	/// Will return a confirmation ID for later use with check_transaction.
	#[rpc(meta, name = "tetsy_postSign")]
	fn post_sign(&self, _: Self::Metadata, _: H160, _: Bytes) -> BoxFuture<Either<U256, ConfirmationResponse>>;

	/// Posts transaction asynchronously.
	/// Will return a transaction ID for later use with check_transaction.
	#[rpc(meta, name = "tetsy_postTransaction")]
	fn post_transaction(&self, _: Self::Metadata, _: TransactionRequest) -> BoxFuture<Either<U256, ConfirmationResponse>>;

	/// Checks the progress of a previously posted request (transaction/sign).
	/// Should be given a valid send_transaction ID.
	#[rpc(name = "tetsy_checkRequest")]
	fn check_request(&self, _: U256) -> Result<Option<ConfirmationResponse>>;

	/// Decrypt some ECIES-encrypted message.
	/// First parameter is the address with which it is encrypted, second is the ciphertext.
	#[rpc(meta, name = "tetsy_decryptMessage")]
	fn decrypt_message(&self, _: Self::Metadata, _: H160, _: Bytes) -> BoxFuture<Bytes>;
}