// RGB Core Library: consensus layer for RGB smart contracts.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2019-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use aluvm::isa::{Bytecode, BytecodeError, ExecStep, InstructionSet};
use aluvm::library::{CodeEofError, LibSite, Read, Write};
use aluvm::reg::CoreRegs;

use super::opcodes::{INSTR_ISAE_FROM, INSTR_ISAE_TO};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(inner)]
#[non_exhaustive]
pub enum TimechainOp {
    Fail,
}

impl InstructionSet for TimechainOp {
    type Context<'ctx> = ();

    fn isa_ids() -> BTreeSet<&'static str> { none!() }

    fn exec(&self, _regs: &mut CoreRegs, _site: LibSite, _context: &Self::Context<'_>) -> ExecStep {
        unreachable!()
    }
}

impl Bytecode for TimechainOp {
    fn byte_count(&self) -> u16 { 0 }

    fn instr_range() -> RangeInclusive<u8> { INSTR_ISAE_FROM..=INSTR_ISAE_TO }

    fn instr_byte(&self) -> u8 { unreachable!() }

    fn encode_args<W>(&self, _writer: &mut W) -> Result<(), BytecodeError>
    where W: Write {
        unreachable!()
    }

    fn decode<R>(_reader: &mut R) -> Result<Self, CodeEofError>
    where
        Self: Sized,
        R: Read,
    {
        unreachable!()
    }
}
