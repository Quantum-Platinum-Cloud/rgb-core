// RGB Core Library: a reference implementation of RGB smart contract standards.
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Components related to the scripting system used by schema or applied at the
//! specific contract node level

use std::collections::BTreeMap;

use amplify::num::u24;
use commit_verify::commit_encode;
use strict_encoding::MediumVec;

/// Virtual machine types.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(Debug)]
pub enum VmType {
    /// Embedded code (not a virtual machine) which is the part of this RGB
    /// Core Library.
    Embedded,

    /// AluVM: pure functional register-based virtual machine designed for RGB
    /// and multiparty computing.
    AluVM,
}

/// Virtual machine and machine-specific script data.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
#[derive(StrictEncode, StrictDecode)]
#[strict_encoding(by_value, repr = u8)]
pub enum VmScript {
    /// Embedded code (not a virtual machine) which is the part of this RGB
    /// Core Library. Using this option results in the fact that the schema
    /// does not commit to the actual validating code and the validation logic
    /// may change in the future (like to be patched) with new RGB Core Lib
    /// releases.
    #[strict_encoding(value = 0x00)]
    Embedded,

    /// AluVM: pure functional register-based virtual machine designed for RGB
    /// and multiparty computing.
    ///
    /// The inner data contains actual executable code in form of complete set
    /// of AliVM libraries, which must be holistic and not dependent on any
    /// external libraries (i.e. must contain all libraries embedded).
    ///
    /// Its routines can be accessed only through well-typed ABI entrance
    /// pointers, defined as a part of the schema.
    #[strict_encoding(value = 0x01)]
    // TODO: Use library-based approach with `aluvm::Lib` type and special
    //       RGB AluVM runtime environment controlling the total number of
    //       libraries used is below 256.
    AluVM(MediumVec<u8>),
}

impl Default for VmScript {
    // TODO: Update default VM type to AluVM in RGBv1 release
    fn default() -> Self { VmScript::Embedded }
}

impl commit_encode::Strategy for VmScript {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl VmScript {
    pub fn vm_type(&self) -> VmType {
        match self {
            VmScript::Embedded => VmType::Embedded,
            VmScript::AluVM(_) => VmType::AluVM,
        }
    }
}

/// VM and script overwrite rules by subschemata.
///
/// Defines whether subschemata are allowed to replace (overwrite) the type of
/// VM and scripts.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "kebab-case")
)]
#[derive(StrictEncode, StrictDecode)]
#[repr(u8)]
pub enum OverrideRules {
    #[display("deny")]
    /// Denies overwrites
    Deny = 0u8,

    #[display("allow-same-vm")]
    /// Allows overwrite only if the same VM is used
    AllowSameVm = 1u8,

    #[display("allow-any-vm")]
    /// Allows overwrite of both executable code and type of VM
    AllowAnyVm = 2u8,
}

impl Default for OverrideRules {
    fn default() -> Self { OverrideRules::Deny }
}

/// All possible procedures which may be called to via ABI table
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(doc_comments)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
#[non_exhaustive]
#[repr(u8)]
pub enum Action {
    /// Genesis validation procedure
    ValidateGenesis = 0,

    /// State transition validation procedure
    ValidateTransition = 2,

    /// State extension validation procedure
    ValidateExtension = 3,

    /// State assignment validation procedure
    ValidateAssignment = 4,

    /// Procedure creating blank state transition, passing set of owned rights
    /// from a given UTXO set to a new UTXO set
    BlankTransition = 0x10,
}

/// Marker trait for all script-based actions, which are the keys in the ABI
/// table
pub trait GenericAction: Sized + Ord + Copy + Into<Action> {}

/// Marker trait for node actions, which are the keys in the ABI table
pub trait NodeAction: GenericAction {}

/// Genesis-specific ABI table keys
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "snake_case")
)]
#[derive(StrictEncode, StrictDecode)]
#[non_exhaustive]
#[repr(u8)]
pub enum GenesisAction {
    #[display("validate")]
    /// Validation of the state & metadata defined by a genesis
    Validate = 0,
}

impl From<GenesisAction> for Action {
    fn from(action: GenesisAction) -> Self {
        match action {
            GenesisAction::Validate => Action::ValidateGenesis,
        }
    }
}

impl GenericAction for GenesisAction {}
impl NodeAction for GenesisAction {}

/// State extension-specific ABI table keys
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "snake_case")
)]
#[derive(StrictEncode, StrictDecode)]
#[non_exhaustive]
#[repr(u8)]
pub enum ExtensionAction {
    #[display("validate")]
    /// Validation of the state & metadata defined by a state extension
    Validate = 0,
}

impl From<ExtensionAction> for Action {
    fn from(action: ExtensionAction) -> Self {
        match action {
            ExtensionAction::Validate => Action::ValidateExtension,
        }
    }
}

impl GenericAction for ExtensionAction {}
impl NodeAction for ExtensionAction {}

/// State transition-specific ABI table keys
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "snake_case")
)]
#[derive(StrictEncode, StrictDecode)]
#[non_exhaustive]
#[repr(u8)]
pub enum TransitionAction {
    #[display("validate")]
    /// Validation of the state & metadata defined by a state transition
    Validate = 0,

    #[display("blank")]
    /// Creation of an empty (blank) state transition transferring data 1-to-1
    /// from a spending UTXO to a new UTXO. Used when other schema spends UTXO
    /// with the rights assigned under this schema.
    GenerateBlank = 1,
}

impl From<TransitionAction> for Action {
    fn from(action: TransitionAction) -> Self {
        match action {
            TransitionAction::Validate => Action::ValidateTransition,
            TransitionAction::GenerateBlank => Action::BlankTransition,
        }
    }
}

impl GenericAction for TransitionAction {}
impl NodeAction for TransitionAction {}

/// ABI table keys for owned right assignment entries (parts of contract nodes)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "snake_case")
)]
#[derive(StrictEncode, StrictDecode)]
#[non_exhaustive]
#[repr(u8)]
pub enum AssignmentAction {
    #[display("validate")]
    /// Validation of the state & metadata defined by a state assignment
    Validate = 0,
}

impl From<AssignmentAction> for Action {
    fn from(action: AssignmentAction) -> Self {
        match action {
            AssignmentAction::Validate => Action::ValidateAssignment,
        }
    }
}

impl GenericAction for AssignmentAction {}

/// Offset within script data for the procedure entry point.
///
/// Part of the ABI data.
///
/// NB: For embedded procedures this is a code name of the embedded procedure
///     as defined by [`EmbeddedProcedure`]
pub type EntryPoint = u24;

/// ABI table for contract genesis
pub type GenesisAbi = BTreeMap<GenesisAction, EntryPoint>;
/// ABI table for contract state extension
pub type ExtensionAbi = BTreeMap<ExtensionAction, EntryPoint>;
/// ABI table for contract state transition
pub type TransitionAbi = BTreeMap<TransitionAction, EntryPoint>;
/// ABI table for owned rights assignment inside a contract node
pub type AssignmentAbi = BTreeMap<AssignmentAction, EntryPoint>;

/// Market trait for generalizing over all available ABI types
pub trait Abi {}

impl Abi for GenesisAbi {}
impl Abi for ExtensionAbi {}
impl Abi for TransitionAbi {}
impl Abi for AssignmentAbi {}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use strict_encoding::strict_serialize;

    use super::*;
    use crate::vm::embedded::AssignmentValidator;

    #[test]
    fn test_basics() {
        // Test Actions and Standard procedures
        // TODO: Uncomment once `test_enum_u8_exhaustive` update to
        //       no-num-trait version will be complete
        /*
        test_enum_u8_exhaustive!(AssignmentAction; AssignmentAction::Validate => 0);
        test_enum_u8_exhaustive!(TransitionAction;
            TransitionAction::Validate => 0,
            TransitionAction::GenerateBlank => 1
        );
        test_enum_u8_exhaustive!(EmbeddedProcedure;
            EmbeddedProcedure::FungibleNoInflation => 0x01,
            EmbeddedProcedure::FungibleIssue => 0x02,
            EmbeddedProcedure::IdentityTransfer => 0x11,
            EmbeddedProcedure::NftIssue => 0x12,
            EmbeddedProcedure::ProofOfBurn => 0x20,
            EmbeddedProcedure::ProofOfReserve => 0x21,
            EmbeddedProcedure::RightsSplit => 0x30
        );*/

        // Test Transition and Assignment ABI
        let mut trans_abi = TransitionAbi::new();
        trans_abi.insert(
            TransitionAction::Validate,
            AssignmentValidator::FungibleNoInflation as EntryPoint,
        );
        assert_eq!(
            vec![0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            strict_serialize(&trans_abi).unwrap()
        );

        let mut assignment_abi = AssignmentAbi::new();
        assignment_abi.insert(AssignmentAction::Validate, 45.try_into().unwrap());
        assert_eq!(
            vec![0x01, 0x00, 0x00, 0x2d, 0x00, 0x00, 0x00],
            strict_serialize(&assignment_abi).unwrap()
        );
    }
}
