//! `MinVec<T, MIN>` is the mechanism for the schema particles with a `minOccurs` lower
//! bound: the check lives in the type, since no serde attribute expresses it.
//!
//! The parse side is tested on the real fields that carry it: `minOccurs >= 2` in
//! `leaf_type_cardinality_test.rs`, `minOccurs = 1` in `required_vec_rejects_empty_test.rs`.
//! This file pins the construction side: a value shorter than `MIN` cannot be built, the
//! infallible constructors keep their items in order, and the bound's error reaches a
//! builder caller with its message intact.

use openscenario_rs::types::basic::MinVec;

#[test]
fn new_rejects_fewer_than_min_items_and_accepts_min() {
    fn short<const MIN: usize>(len: usize) -> String {
        MinVec::<u8, MIN>::new(vec![0; len])
            .unwrap_err()
            .to_string()
    }
    assert_eq!(
        short::<1>(0),
        "Validation error in field 'MinVec': expected at least 1 items, got 0"
    );
    assert_eq!(
        short::<2>(0),
        "Validation error in field 'MinVec': expected at least 2 items, got 0"
    );
    assert_eq!(
        short::<2>(1),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
    assert_eq!(
        short::<3>(2),
        "Validation error in field 'MinVec': expected at least 3 items, got 2"
    );

    assert_eq!(MinVec::<u8, 1>::new(vec![1]).unwrap().as_slice(), &[1]);
    assert_eq!(
        MinVec::<u8, 2>::new(vec![1, 2]).unwrap().as_slice(),
        &[1, 2]
    );
    assert_eq!(
        MinVec::<u8, 3>::new(vec![1, 2, 3]).unwrap().as_slice(),
        &[1, 2, 3]
    );

    // `TryFrom<Vec<T>>` is the same check under the standard conversion trait.
    assert!(MinVec::<u8, 2>::try_from(vec![1]).is_err());
    assert_eq!(
        MinVec::<u8, 2>::try_from(vec![1, 2]).unwrap().into_inner(),
        vec![1, 2]
    );
}

#[test]
fn from_min_and_push_build_without_a_result() {
    // `from_min` takes exactly `MIN` guaranteed items as an array, so it returns
    // `MinVec` directly: `MinVec<u8, 2>::from_min([1], ..)` does not compile.
    let one: MinVec<u8, 1> = MinVec::from_min([1], vec![]);
    assert_eq!(one.as_slice(), &[1]);

    let several: MinVec<u8, 1> = MinVec::from_min([1], vec![2, 3]);
    assert_eq!(several.as_slice(), &[1, 2, 3]);

    let mut two: MinVec<u8, 2> = MinVec::from_min([1, 2], vec![3]);
    assert_eq!(two.as_slice(), &[1, 2, 3]);

    // `push` grows a live value; it cannot fall below the bound, so it has no `Result`.
    two.push(4);
    two.push(5);
    assert_eq!(two.as_slice(), &[1, 2, 3, 4, 5]);
}

/// The builders call `MinVec::new(..)?` inside `build()`, which returns `BuilderResult`.
/// `BuilderError: From<openscenario_rs::error::Error>` is what makes that `?` compile,
/// and the bound's message must survive the conversion.
#[cfg(feature = "builder")]
#[test]
fn a_min_vec_error_converts_into_a_builder_error_with_its_message() {
    use openscenario_rs::builder::BuilderError;

    let err: BuilderError = MinVec::<u8, 2>::new(vec![1]).unwrap_err().into();
    assert!(matches!(err, BuilderError::OpenScenarioError(_)));
    assert_eq!(
        err.to_string(),
        "OpenSCENARIO error: Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}
