#![allow(clippy::from_over_into)]
use frame_support::{
    assert_noop, assert_ok, dispatch::DispatchResult, sp_io, sp_std::ops::RangeInclusive,
    Blake2_256, StorageHasher,
};

use crate::bond::transfer_bond_units;
use crate::tests::mock::*;
use crate::{
    BondId, BondImpactReportStruct, BondInnerStructOf, BondPeriodNumber, BondState, BondStructOf,
    BondUnitAmount, BondUnitPackage, BondUnitSaleLotStructOf, Error, EverUSDBalance, Module,
    AUDITOR_ROLE_MASK, DEFAULT_DAY_DURATION, ISSUER_ROLE_MASK, MASTER_ROLE_MASK,
};

type Evercity = Module<TestRuntime>;
type Timestamp = pallet_timestamp::Module<TestRuntime>;
type Moment = <TestRuntime as pallet_timestamp::Config>::Moment;
type BondInnerStruct = BondInnerStructOf<TestRuntime>;
type BondStruct = BondStructOf<TestRuntime>;
type RuntimeError = Error<TestRuntime>;
type AccountId = <TestRuntime as frame_system::Config>::AccountId;
type BondUnitSaleLotStruct = BondUnitSaleLotStructOf<TestRuntime>;