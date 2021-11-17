// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::UnexpectedEOF, unpacker::Unpacker};

/// An [`Unpacker`] backed by a `&[u8]`.
pub struct SliceUnpacker<'u>(&'u [u8]);

impl<'u> SliceUnpacker<'u> {
    /// Creates a new unpacker from a byte slice.
    pub fn new(slice: &'u [u8]) -> Self {
        Self(slice)
    }
}

impl<'u> Unpacker for SliceUnpacker<'u> {
    type Error = UnexpectedEOF;

    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, mut bytes: B) -> Result<(), Self::Error> {
        let slice = bytes.as_mut();
        let len = slice.len();

        if self.0.len() >= len {
            let (head, tail) = self.0.split_at(len);
            self.0 = tail;
            slice.copy_from_slice(head);
            Ok(())
        } else {
            Err(UnexpectedEOF {
                required: len,
                had: self.0.len(),
            })
        }
    }
}