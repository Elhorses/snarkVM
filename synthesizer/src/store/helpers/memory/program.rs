// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::store::{helpers::memory::MemoryMap, FinalizeStorage};
use console::{
    prelude::*,
    program::{Identifier, Plaintext, ProgramID, Value},
    types::Field,
};

use indexmap::{IndexMap, IndexSet};

/// An in-memory program state storage.
#[derive(Clone)]
pub struct FinalizeMemory<N: Network> {
    /// The program ID map.
    program_id_map: MemoryMap<ProgramID<N>, IndexSet<Identifier<N>>>,
    /// The mapping ID map.
    mapping_id_map: MemoryMap<(ProgramID<N>, Identifier<N>), Field<N>>,
    /// The key-value ID map.
    key_value_id_map: MemoryMap<Field<N>, IndexMap<Field<N>, Field<N>>>,
    /// The key map.
    key_map: MemoryMap<Field<N>, Plaintext<N>>,
    /// The value map.
    value_map: MemoryMap<Field<N>, Value<N>>,
    /// The optional development ID.
    dev: Option<u16>,
}

#[rustfmt::skip]
impl<N: Network> FinalizeStorage<N> for FinalizeMemory<N> {
    type ProgramIDMap = MemoryMap<ProgramID<N>, IndexSet<Identifier<N>>>;
    type MappingIDMap = MemoryMap<(ProgramID<N>, Identifier<N>), Field<N>>;
    type KeyValueIDMap = MemoryMap<Field<N>, IndexMap<Field<N>, Field<N>>>;
    type KeyMap = MemoryMap<Field<N>, Plaintext<N>>;
    type ValueMap = MemoryMap<Field<N>, Value<N>>;

    /// Initializes the program state storage.
    fn open(dev: Option<u16>) -> Result<Self> {
        Ok(Self {
            program_id_map: MemoryMap::default(),
            mapping_id_map: MemoryMap::default(),
            key_value_id_map: MemoryMap::default(),
            key_map: MemoryMap::default(),
            value_map: MemoryMap::default(),
            dev,
        })
    }

    /// Returns the program ID map.
    fn program_id_map(&self) -> &Self::ProgramIDMap {
        &self.program_id_map
    }

    /// Returns the mapping ID map.
    fn mapping_id_map(&self) -> &Self::MappingIDMap {
        &self.mapping_id_map
    }

    /// Returns the key-value ID map.
    fn key_value_id_map(&self) -> &Self::KeyValueIDMap {
        &self.key_value_id_map
    }

    /// Returns the key map.
    fn key_map(&self) -> &Self::KeyMap {
        &self.key_map
    }

    /// Returns the value map.
    fn value_map(&self) -> &Self::ValueMap {
        &self.value_map
    }

    /// Returns the optional development ID.
    fn dev(&self) -> Option<u16> {
        self.dev
    }
}