// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    error::{MessageUnpackError, ValidationError},
    parents::{Parent, Parents, MESSAGE_PARENTS_RANGE},
};
use bee_packable::{Packable, UnpackError};
use bee_test::rand::{
    message::{
        parents::{rand_parent, rand_parents},
        rand_message_id,
    },
    vec::rand_vec,
};

#[test]
fn new_invalid_less_than_min() {
    let min = MESSAGE_PARENTS_RANGE.start() - 1;

    assert!(matches!(
        Parents::new(rand_vec(rand_parent, min)),
        Err(ValidationError::InvalidParentsCount(m)) if m == min
    ));
}

#[test]
fn new_invalid_more_than_max() {
    let max = MESSAGE_PARENTS_RANGE.end() + 1;

    assert!(matches!(
        Parents::new(rand_vec(rand_parent, max)),
        Err(ValidationError::InvalidParentsCount(m)) if m == max
    ));
}

#[test]
fn new_invalid_not_sorted() {
    let mut inner = rand_vec(rand_parent, 8);
    inner.reverse();

    assert!(matches!(
        Parents::new(inner),
        Err(ValidationError::ParentsNotUniqueSorted)
    ));
}

#[test]
fn new_invalid_not_unique() {
    let mut inner = rand_vec(rand_parent, 7);
    inner.sort();
    inner.push(*inner.last().unwrap());

    assert!(matches!(
        Parents::new(inner),
        Err(ValidationError::ParentsNotUniqueSorted)
    ));
}

#[test]
fn new_invalid_no_strong_parents() {
    let mut inner = (0..8).map(|_| Parent::Weak(rand_message_id())).collect::<Vec<_>>();
    inner.sort();

    assert!(matches!(
        Parents::new(inner),
        Err(ValidationError::InvalidStrongParentsCount(0)),
    ));
}

#[test]
fn packed_len() {
    let mut inner = rand_vec(rand_parent, 5);
    inner.sort();

    let parents = Parents::new(inner).unwrap();

    assert_eq!(parents.packed_len(), 1 + 1 + 5 * 32);
    assert_eq!(parents.pack_to_vec().unwrap().len(), 1 + 1 + 5 * 32);
}

#[test]
fn pack_unpack_valid() {
    let parents_1 = rand_parents();
    let parents_2 = Parents::unpack_from_slice(parents_1.pack_to_vec().unwrap()).unwrap();

    assert_eq!(parents_1, parents_2);
}

#[test]
fn unpack_invalid_less_than_min() {
    let bytes = vec![0, 1];

    assert!(matches!(
        Parents::unpack_from_slice(bytes),
        Err(UnpackError::Packable(MessageUnpackError::Validation(
            ValidationError::InvalidParentsCount(0)
        ))),
    ));
}

#[test]
fn unpack_invalid_more_than_max() {
    let bytes = vec![
        9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
    ];

    assert!(matches!(
        Parents::unpack_from_slice(bytes),
        Err(UnpackError::Packable(MessageUnpackError::Validation(
            ValidationError::InvalidParentsCount(9)
        ))),
    ));
}

#[test]
fn unpack_invalid_no_strong_parents() {
    let bytes = vec![
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];

    assert!(matches!(
        Parents::unpack_from_slice(bytes),
        Err(UnpackError::Packable(MessageUnpackError::Validation(
            ValidationError::InvalidStrongParentsCount(0)
        ))),
    ));
}

#[test]
fn unpack_invalid_not_sorted() {
    let mut inner = rand_vec(rand_message_id, 8);
    inner.reverse();

    // Remove 8 byte vector length field and replace with 1 byte, to represent message parents, and one byte for parent
    // types.
    let mut packed = vec![8, 1];
    let mut packed_messages = inner
        .pack_to_vec()
        .unwrap()
        .split_at(core::mem::size_of::<u64>())
        .1
        .to_vec();
    packed.append(&mut packed_messages);

    assert!(matches!(
        Parents::unpack_from_slice(packed),
        Err(UnpackError::Packable(MessageUnpackError::Validation(
            ValidationError::ParentsNotUniqueSorted
        ))),
    ));
}

#[test]
fn unpack_invalid_not_unique() {
    let mut inner = rand_vec(rand_message_id, 7);
    inner.sort();
    inner.push(*inner.last().unwrap());

    // Remove 8 byte vector length field and replace with 1 byte, to represent message parents, and one byte for parent
    // types.
    let mut packed = vec![8, 1];
    let mut packed_messages = inner
        .pack_to_vec()
        .unwrap()
        .split_at(core::mem::size_of::<u64>())
        .1
        .to_vec();
    packed.append(&mut packed_messages);

    assert!(matches!(
        Parents::unpack_from_slice(packed),
        Err(UnpackError::Packable(MessageUnpackError::Validation(
            ValidationError::ParentsNotUniqueSorted
        ))),
    ));
}
