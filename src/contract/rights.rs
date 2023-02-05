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

use core::fmt::Debug;
use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet};

use amplify::Wrapper;
use commit_verify::merkle::MerkleNode;

use super::{NodeId, TypedAssignments, EMPTY_ASSIGNMENTS};
use crate::schema;

/// Holds definition of valencies for contract nodes, which is a set of
/// allowed valencies types
pub(crate) type PublicRightsInner = BTreeSet<schema::PublicRightType>;
pub(crate) type OwnedRightsInner = BTreeMap<schema::OwnedRightType, TypedAssignments>;
pub(crate) type ParentOwnedRightsInner =
    BTreeMap<NodeId, BTreeMap<schema::OwnedRightType, Vec<u16>>>;
pub(crate) type ParentPublicRightsInner = BTreeMap<NodeId, BTreeSet<schema::PublicRightType>>;

#[derive(Wrapper, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, From)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct OwnedRights(OwnedRightsInner);

impl OwnedRights {
    pub fn iter(&self) -> btree_map::Iter<'_, schema::OwnedRightType, TypedAssignments> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> btree_map::IterMut<'_, schema::OwnedRightType, TypedAssignments> {
        self.0.iter_mut()
    }

    pub fn assignments_by_type(
        &self,
        owned_rights_type: schema::OwnedRightType,
    ) -> &TypedAssignments {
        self.0.get(&owned_rights_type).unwrap_or(&EMPTY_ASSIGNMENTS)
    }
}

impl IntoIterator for OwnedRights {
    type Item = <OwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <OwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a OwnedRights {
    type Item = <&'a OwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a OwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a> IntoIterator for &'a mut OwnedRights {
    type Item = <&'a mut OwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a mut OwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct PublicRights(PublicRightsInner);

impl PublicRights {
    pub fn iter(&self) -> btree_set::Iter<'_, schema::PublicRightType> { self.0.iter() }
}

impl IntoIterator for PublicRights {
    type Item = <PublicRightsInner as IntoIterator>::Item;
    type IntoIter = <PublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a PublicRights {
    type Item = <&'a PublicRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a PublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct ParentOwnedRights(ParentOwnedRightsInner);

impl ParentOwnedRights {
    pub fn iter(&self) -> btree_map::Iter<'_, NodeId, BTreeMap<schema::OwnedRightType, Vec<u16>>> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> btree_map::IterMut<'_, NodeId, BTreeMap<schema::OwnedRightType, Vec<u16>>> {
        self.0.iter_mut()
    }
}

impl IntoIterator for ParentOwnedRights {
    type Item = <ParentOwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <ParentOwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a ParentOwnedRights {
    type Item = <&'a ParentOwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a ParentOwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a> IntoIterator for &'a mut ParentOwnedRights {
    type Item = <&'a mut ParentOwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a mut ParentOwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct ParentPublicRights(ParentPublicRightsInner);

impl ParentPublicRights {
    pub fn iter(&self) -> btree_map::Iter<'_, NodeId, BTreeSet<schema::PublicRightType>> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> btree_map::IterMut<'_, NodeId, BTreeSet<schema::PublicRightType>> {
        self.0.iter_mut()
    }
}

impl IntoIterator for ParentPublicRights {
    type Item = <ParentPublicRightsInner as IntoIterator>::Item;
    type IntoIter = <ParentPublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a ParentPublicRights {
    type Item = <&'a ParentPublicRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a ParentPublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a> IntoIterator for &'a mut ParentPublicRights {
    type Item = <&'a mut ParentPublicRightsInner as IntoIterator>::Item;
    type IntoIter = <&'a mut ParentPublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PublicRightsLeaf(pub schema::PublicRightType);

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct OwnedRightsLeaf(pub schema::OwnedRightType, pub MerkleNode);

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ParentPublicRightsLeaf(pub NodeId, pub schema::PublicRightType);

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ParentOwnedRightsLeaf(pub NodeId, pub schema::OwnedRightType, pub u16);
