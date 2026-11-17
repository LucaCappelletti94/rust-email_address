//!
//! Tests for the `diesel_support` feature of the `email_address` crate.
//!
//! Added by GitHub issue #58: [Optional `diesel` feature for `EmailAddress` columns](https://github.com/johnstonskj/rust-email_address/issues/58)
//!
#![cfg(feature = "diesel")]

use claims::{assert_err, assert_matches, assert_ok, assert_ok_eq};
use core::str::FromStr;
use diesel::{
    prelude::*,
    result::{DeserializeFieldError, Error as DieselError},
    sql_types::{Nullable, Text},
};
use email_address::{EmailAddress, Error};

fn connection() -> SqliteConnection {
    assert_ok!(SqliteConnection::establish(":memory:"))
}

#[test]
// Feature test: GitHub issue #58
fn test_round_trip() {
    let email = assert_ok!(EmailAddress::from_str("simple@example.com"));
    let mut connection = connection();

    assert_ok_eq!(
        diesel::select((&email).into_sql::<Text>()).get_result::<EmailAddress>(&mut connection),
        email
    );
}

#[test]
// Feature test: GitHub issue #58
fn test_round_trip_nullable() {
    let email = assert_ok!(EmailAddress::from_str("simple@example.com"));
    let mut connection = connection();

    assert_ok_eq!(
        diesel::select(Some(&email).into_sql::<Nullable<Text>>())
            .get_result::<Option<EmailAddress>>(&mut connection),
        Some(email)
    );
}

#[test]
// Feature test: GitHub issue #58
fn test_read_invalid_value() {
    let mut connection = connection();
    let error = assert_err!(
        diesel::select("Abc.example.com".into_sql::<Text>())
            .get_result::<EmailAddress>(&mut connection)
    );
    let DieselError::DeserializationError(boxed_error) = error else {
        panic!("expected a deserialization error, got {error:?}");
    };
    let field_error = assert_ok!(boxed_error.downcast::<DeserializeFieldError>());
    let error = assert_ok!(field_error.error.downcast::<Error>());

    assert_matches!(*error, Error::MissingSeparator);
}
