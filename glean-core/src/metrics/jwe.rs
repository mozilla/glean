// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::str::FromStr;

use serde::Serialize;

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::{Metric, MetricType};
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

const DEFAULT_MAX_CHARS_PER_ELEMENT: usize = 1024;

/// Verifies if a string is [`BASE64URL`](https://tools.ietf.org/html/rfc4648#section-5) compliant.
///
/// As such, the string must match the regex: `[a-zA-Z0-9\-\_]*`.
///
/// > **Note** As described in the [JWS specification](https://tools.ietf.org/html/rfc7516#section-2),
/// > the BASE64URL encoding used by JWE discards any padding,
/// > that is why we can ignore that for this validation.
///
/// The regex crate isn't used here because it adds to the binary size,
/// and the Glean SDK doesn't use regular expressions anywhere else.
fn validate_base64url_encoding(value: &str) -> bool {
    let mut iter = value.chars();

    loop {
        match iter.next() {
            // We are done, so the whole expression is valid.
            None => return true,
            // Valid characters.
            Some('_') | Some('-') | Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') => (),
            // An invalid character.
            Some(_) => return false,
        }
    }
}

/// Representation of a [JWE](https://tools.ietf.org/html/rfc7516).
#[derive(Serialize)]
struct Jwe {
    header: String,
    key: String,
    init_vector: String,
    cipher_text: String,
    auth_tag: String,
}

impl Jwe {
    /// Create a new JWE and validate all elements.
    ///
    /// Validation includes checking if each element is valid BASE64URL according the the JWE specification
    /// and also checking if each element does not exceed DEFAULT_MAX_CHARS_PER_ELEMENT.
    ///
    /// **Note** The character limit is our own constraint, not part of the spec.
    ///
    /// ## Arguments
    ///
    /// * `header` - the JWE Protected Header element.
    /// * `key` - the JWE Encrypted Key element. May be empy.
    /// * `init_vector` - the JWE Initialization Vector element.  May be empy.
    /// * `cipher_text` - the JWE Ciphertext element.
    /// * `auth_tag` - the JWE Authentication Tag element.  May be empy.
    fn new<S: Into<String>>(
        header: S,
        key: S,
        init_vector: S,
        cipher_text: S,
        auth_tag: S,
    ) -> Result<Self, (ErrorType, &'static str)> {
        let header = header.into();
        let key = key.into();
        let init_vector = init_vector.into();
        let cipher_text = cipher_text.into();
        let auth_tag = auth_tag.into();

        for element in [&header, &cipher_text].iter() {
            if element.is_empty() {
                return Err((
                    ErrorType::InvalidValue,
                    "Elements `header` and `cipher_text` must not be empty.",
                ));
            }
        }

        for element in [&header, &key, &init_vector, &cipher_text, &auth_tag].iter() {
            if element.len() > DEFAULT_MAX_CHARS_PER_ELEMENT {
                return Err((
                    ErrorType::InvalidOverflow,
                    "Element in JWE value exceeds maximum number of characters.",
                ));
            }

            if !validate_base64url_encoding(&element) {
                return Err((
                    ErrorType::InvalidValue,
                    "Element in JWE value is not valid BASE64URL.",
                ));
            }
        }

        Ok(Self {
            header,
            key,
            init_vector,
            cipher_text,
            auth_tag,
        })
    }
}

/// Trait implementation to convert a JWE [`compact representation`](https://tools.ietf.org/html/rfc7516#appendix-A.2.7) string into a Jwe struct.
impl FromStr for Jwe {
    type Err = (ErrorType, &'static str);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements: Vec<&str> = s.split('.').collect();

        if elements.len() != 5 {
            return Err((
                ErrorType::InvalidValue,
                "JWE value is not formatted as expected.",
            ));
        }

        // Consume the vector extracting each part of the JWE from it.
        //
        // Safe unwraps, we already defined that the slice has five elements.
        let auth_tag = elements.pop().unwrap();
        let cipher_text = elements.pop().unwrap();
        let init_vector = elements.pop().unwrap();
        let key = elements.pop().unwrap();
        let header = elements.pop().unwrap();

        Self::new(header, key, init_vector, cipher_text, auth_tag)
    }
}

/// Trait implementation to print the Jwe struct as the proper JWE [`compact representation`](https://tools.ietf.org/html/rfc7516#appendix-A.2.7).
impl fmt::Display for Jwe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.header, self.key, self.init_vector, self.cipher_text, self.auth_tag
        )
    }
}

/// A JWE metric.
///
/// This metric will be work as a "transport" for JWE encrypted data.
///
/// The actual encrypti on is done somewhere else,
/// Glean must only make sure the data is valid JWE.
#[derive(Clone, Debug)]
pub struct JweMetric {
    meta: CommonMetricData,
}

impl MetricType for JweMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl JweMetric {
    /// Create a new JWE metric.
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    /// Set to the specified JWE value.
    ///
    /// ## Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `value` - the [`compact representation`](https://tools.ietf.org/html/rfc7516#appendix-A.2.7) of a JWE value.
    pub fn set_with_compact_repr<S: Into<String>>(&self, glean: &Glean, value: S) {
        if !self.should_record(glean) {
            return;
        }

        let value = value.into();
        match Jwe::from_str(&value) {
            Ok(_) => glean
                .storage()
                .record(glean, &self.meta, &Metric::Jwe(value)),
            Err((error_type, msg)) => record_error(glean, &self.meta, error_type, msg, None),
        };
    }

    /// Build a JWE value from it's elements and set to it.
    ///
    /// ## Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `header` - the JWE Protected Header element.
    /// * `key` - the JWE Encrypted Key element.
    /// * `init_vector` - the JWE Initialization Vector element.
    /// * `cipher_text` - the JWE Ciphertext element.
    /// * `auth_tag` - the JWE Authentication Tag element.
    pub fn set<S: Into<String>>(
        &self,
        glean: &Glean,
        header: S,
        key: S,
        init_vector: S,
        cipher_text: S,
        auth_tag: S,
    ) {
        if !self.should_record(glean) {
            return;
        }

        match Jwe::new(header, key, init_vector, cipher_text, auth_tag) {
            Ok(jwe) => glean
                .storage()
                .record(glean, &self.meta, &Metric::Jwe(jwe.to_string())),
            Err((error_type, msg)) => record_error(glean, &self.meta, error_type, msg, None),
        };
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as a string.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<String> {
        match StorageManager.snapshot_metric(
            glean.storage(),
            storage_name,
            &self.meta.identifier(glean),
        ) {
            Some(Metric::Jwe(b)) => Some(b),
            _ => None,
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored JWE as a JSON String of the serialized value.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_json_string(
        &self,
        glean: &Glean,
        storage_name: &str,
    ) -> Option<String> {
        self.test_get_value(glean, storage_name).map(|snapshot| {
            serde_json::to_string(
                &Jwe::from_str(&snapshot).expect("Stored JWE metric should be valid JWE value."),
            )
            .unwrap()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const HEADER: &str = "eyJhbGciOiJFQ0RILUVTIiwia2lkIjoiMFZFRTdmT0txbFdHVGZrY0taRUJ2WWl3dkpMYTRUUGlJVGxXMGJOcDdqVSIsImVwayI6eyJrdHkiOiJFQyIsImNydiI6IlAtMjU2IiwieCI6InY3Q1FlRWtVQjMwUGwxV0tPMUZUZ25OQlNQdlFyNlh0UnZxT2kzSWdzNHciLCJ5IjoiNDBKVEpaQlMwOXpWNHpxb0hHZDI5NGFDeHRqcGU5a09reGhELVctUEZsSSJ9LCJlbmMiOiJBMjU2R0NNIn0";
    const KEY: &str = "";
    const INIT_VECTOR: &str = "A_wzJya943vlHKFH";
    const CIPHER_TEXT: &str = "yq0JhkGZiZd6UiZK6goTcEf6i4gbbBeXxvq8QV5_nC4";
    const AUTH_TAG: &str = "Knl_sYSBrrP-aa54z6B6gA";
    const JWE: &str = "eyJhbGciOiJFQ0RILUVTIiwia2lkIjoiMFZFRTdmT0txbFdHVGZrY0taRUJ2WWl3dkpMYTRUUGlJVGxXMGJOcDdqVSIsImVwayI6eyJrdHkiOiJFQyIsImNydiI6IlAtMjU2IiwieCI6InY3Q1FlRWtVQjMwUGwxV0tPMUZUZ25OQlNQdlFyNlh0UnZxT2kzSWdzNHciLCJ5IjoiNDBKVEpaQlMwOXpWNHpxb0hHZDI5NGFDeHRqcGU5a09reGhELVctUEZsSSJ9LCJlbmMiOiJBMjU2R0NNIn0..A_wzJya943vlHKFH.yq0JhkGZiZd6UiZK6goTcEf6i4gbbBeXxvq8QV5_nC4.Knl_sYSBrrP-aa54z6B6gA";

    #[test]
    fn generates_jwe_from_compact_repr() {
        // Errors if cipher_text and auth_tag are empty
        if let Err((error_type, _)) =
            Jwe::from_str(&format!(".{}.{}..{}", KEY, INIT_VECTOR, AUTH_TAG))
        {
            assert_eq!(error_type, ErrorType::InvalidValue);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Errors if one of the parts is not valid BASE64URL
        let invalid = "inv@alid value";
        if let Err((error_type, _)) =
            Jwe::from_str(&format!("{}...{}.{}", HEADER, CIPHER_TEXT, invalid))
        {
            assert_eq!(error_type, ErrorType::InvalidValue);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Errors if one of the parts exceeds max length
        let too_long = (0..1025).map(|_| "X").collect::<String>();
        if let Err((error_type, _)) =
            Jwe::from_str(&format!("{}...{}.{}", HEADER, CIPHER_TEXT, too_long))
        {
            assert_eq!(error_type, ErrorType::InvalidOverflow);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Doesn't error if allowed elements are empty and puts the elements in the right places
        let jwe = Jwe::from_str(&format!("{}...{}.", HEADER, CIPHER_TEXT)).unwrap();
        assert_eq!(jwe.header, HEADER);
        assert_eq!(jwe.key, "");
        assert_eq!(jwe.init_vector, "");
        assert_eq!(jwe.cipher_text, CIPHER_TEXT);
        assert_eq!(jwe.auth_tag, "");

        // Doesn't error for the sample value
        let jwe = Jwe::from_str(JWE).unwrap();
        assert_eq!(jwe.header, HEADER);
        assert_eq!(jwe.key, KEY);
        assert_eq!(jwe.init_vector, INIT_VECTOR);
        assert_eq!(jwe.cipher_text, CIPHER_TEXT);
        assert_eq!(jwe.auth_tag, AUTH_TAG);
    }

    #[test]
    fn generates_jwe_from_elements() {
        // Errors if cipher_text and auth_tag are empty
        if let Err((error_type, _)) = Jwe::new("", KEY, INIT_VECTOR, "", AUTH_TAG) {
            assert_eq!(error_type, ErrorType::InvalidValue);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Errors if one of the parts is not valid BASE64URL
        let invalid = "inv@alid value";
        if let Err((error_type, _)) = Jwe::new(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, invalid) {
            assert_eq!(error_type, ErrorType::InvalidValue);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Errors if one of the parts exceeds max length
        let too_long = (0..1025).map(|_| "X").collect::<String>();
        if let Err((error_type, _)) = Jwe::new(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, &too_long) {
            assert_eq!(error_type, ErrorType::InvalidOverflow);
        } else {
            panic!("Should not have built JWE successfully.")
        }

        // Doesn't error if allowed elements are empty
        assert!(Jwe::new(HEADER, "", "", CIPHER_TEXT, "").is_ok());

        // Doesn't error for the sample value
        assert!(Jwe::new(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG).is_ok());
    }

    #[test]
    fn tranforms_jwe_struct_to_string_correctly() {
        let jwe = Jwe::from_str(JWE).unwrap();
        assert_eq!(jwe.to_string(), JWE);
    }

    #[test]
    fn validates_base64url_correctly() {
        assert!(validate_base64url_encoding(
            "0987654321AaBbCcDdEeFfGgHhIiKkLlMmNnOoPpQqRrSsTtUuVvXxWwYyZz-_"
        ));
        assert!(validate_base64url_encoding(""));
        assert!(!validate_base64url_encoding("aa aa"));
        assert!(!validate_base64url_encoding("aa.aa"));
        assert!(!validate_base64url_encoding("!nv@lid-val*e"));
    }
}
