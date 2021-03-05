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

//! This crate defines the Engine trait and related types.

mod enjen;
pub mod signer;

pub use crate::engine::{
	Engine,
	EpochVerifier,
	StateDependentProof,
	ConstructedVerifier,
	EpochChange,
	Proof,
	SystemCall,
	SystemOrCodeCall,
	SystemOrCodeCallKind,
	default_system_or_code_call,
};

#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
