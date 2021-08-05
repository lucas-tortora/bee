// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{address::Address, payload::PAYLOAD_LENGTH_MAX, MessageUnpackError, ValidationError};

use bee_packable::{
    coerce::*, error::UnpackPrefixError, BoundedU32, InvalidBoundedU32, PackError, Packable, Packer, UnpackError,
    Unpacker, VecPrefix,
};

use alloc::vec::Vec;
use core::convert::{Infallible, TryInto};

/// No [`Vec`] max length specified, so use [`PAYLOAD_LENGTH_MAX`] / [`AssetId::LENGTH`].
const PREFIXED_BALANCES_LENGTH_MAX: u32 = PAYLOAD_LENGTH_MAX / (AssetId::LENGTH + core::mem::size_of::<u64>()) as u32;

fn unpack_prefix_to_validation_error(error: UnpackPrefixError<Infallible>) -> ValidationError {
    match error {
        UnpackPrefixError::InvalidPrefixLength(len) => ValidationError::InvalidAssetBalanceCount(len),
        UnpackPrefixError::Packable(e) => match e {},
    }
}

/// Tokenized asset identifier.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetId([u8; Self::LENGTH]);

impl AssetId {
    /// The length (in bytes) of an [`AssetId`].
    pub const LENGTH: usize = 32;

    /// Creates a new [`AssetId`].
    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }
}

impl From<[u8; Self::LENGTH]> for AssetId {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }
}

/// Tokenized asset balance information.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetBalance {
    /// The ID of the tokenized asset.
    id: AssetId,
    /// The balance of the tokenized asset.
    balance: u64,
}

impl AssetBalance {
    /// Creates a new [`AssetBalance`].
    pub fn new(id: AssetId, balance: u64) -> Self {
        Self { id, balance }
    }

    /// Returns the ID of an [`AssetBalance`].
    pub fn id(&self) -> &AssetId {
        &self.id
    }

    /// Returns the balance of an [`AssetBalance`].
    pub fn balance(&self) -> u64 {
        self.balance
    }
}

/// An output type which can be unlocked via a signature. It deposits onto one single address.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct SignatureLockedAssetOutput {
    address: Address,
    balances: VecPrefix<AssetBalance, BoundedU32<0, PREFIXED_BALANCES_LENGTH_MAX>>,
}

impl SignatureLockedAssetOutput {
    /// The output kind of a [`SignatureLockedAssetOutput`].
    pub const KIND: u8 = 1;

    /// Creates a new [`SignatureLockedAssetOutput`].
    pub fn new(address: Address, balances: Vec<AssetBalance>) -> Result<Self, ValidationError> {
        Ok(Self {
            address,
            balances: balances
                .try_into()
                .map_err(|err: InvalidBoundedU32<0, PREFIXED_BALANCES_LENGTH_MAX>| {
                    ValidationError::InvalidAssetBalanceCount(err.0 as usize)
                })?,
        })
    }

    /// Returns the address of a [`SignatureLockedAssetOutput`].
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Returns the amount of a [`SignatureLockedAssetOutput`].
    pub fn balance_iter(&self) -> impl Iterator<Item = &AssetBalance> {
        self.balances.iter()
    }
}

impl Packable for SignatureLockedAssetOutput {
    type PackError = Infallible;
    type UnpackError = MessageUnpackError;

    fn packed_len(&self) -> usize {
        self.address.packed_len() + 0u32.packed_len() + self.balances.len() * (AssetId::LENGTH + 0u64.packed_len())
    }

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), PackError<Self::PackError, P::Error>> {
        self.address.pack(packer)?;
        self.balances.pack(packer).infallible()
    }

    fn unpack<U: Unpacker>(unpacker: &mut U) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let address = Address::unpack(unpacker).coerce()?;

        let balances = VecPrefix::<AssetBalance, BoundedU32<0, PREFIXED_BALANCES_LENGTH_MAX>>::unpack(unpacker)
            .map_err(|err| match err {
                UnpackError::Packable(err) => UnpackError::Packable(unpack_prefix_to_validation_error(err).into()),
                UnpackError::Unpacker(err) => UnpackError::Unpacker(err),
            })?;

        Ok(Self { address, balances })
    }
}