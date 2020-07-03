// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Ping request representation.

use std::collections::HashMap;

use chrono::prelude::{DateTime, Utc};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde_json::{self, Value as JsonValue};
use std::io::prelude::*;

use crate::system;

/// Creates a formatted date string that can be used with Date headers.
fn create_date_header_value(current_time: DateTime<Utc>) -> String {
    // Date headers are required to be in the following format:
    //
    // <day-name>, <day> <month> <year> <hour>:<minute>:<second> GMT
    //
    // as documented here:
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Date
    // Unfortunately we can't use `current_time.to_rfc2822()` as it
    // formats as "Mon, 22 Jun 2020 10:40:34 +0000", with an ending
    // "+0000" instead of "GMT". That's why we need to go with manual
    // formatting.
    current_time.format("%a, %d %b %Y %T GMT").to_string()
}

fn create_user_agent_header_value(
    version: &str,
    language_binding_name: &str,
    system: &str,
) -> String {
    format!(
        "Glean/{} ({} on {})",
        version, language_binding_name, system
    )
}

/// Represents a request to upload a ping.
#[derive(PartialEq, Debug, Clone)]
pub struct PingRequest {
    /// The Job ID to identify this request,
    /// this is the same as the ping UUID.
    pub document_id: String,
    /// The path for the server to upload the ping to.
    pub path: String,
    /// The body of the request, as a byte array. If gzip encoded, then
    /// the `headers` list will contain a `Content-Encoding` header with
    /// the value `gzip`.
    pub body: Vec<u8>,
    /// A map with all the headers to be sent with the request.
    pub headers: HashMap<&'static str, String>,
}

impl PingRequest {
    /// Creates a new PingRequest.
    ///
    /// Automatically creates the default request headers.
    ///
    /// ## Arguments
    ///
    /// * `document_id` - The UUID of the ping in question.
    /// * `path` - The path to upload this ping to. The format should be `/submit/<app_id>/<ping_name>/<schema_version/<doc_id>`.
    /// * `body` - A JSON object with the contents of the ping in question.
    /// * `debug_view_tag` - The value of the `X-Debug-Id` header, if this is `None` the header is not added.
    pub fn new(
        document_id: &str,
        path: &str,
        body: JsonValue,
        language_binding_name: &str,
        debug_view_tag: Option<&String>,
    ) -> Self {
        // We want uploads to be gzip'd. Instead of doing this for each platform
        // we have language bindings for, apply compression here.
        let original_as_string = body.to_string();
        let gzipped_content = Self::gzip_content(path, original_as_string.as_bytes());
        let add_gzip_header = gzipped_content.is_some();
        let body = gzipped_content.unwrap_or_else(|| original_as_string.into_bytes());
        let body_len = body.len();

        Self {
            document_id: document_id.into(),
            path: path.into(),
            body,
            headers: Self::create_request_headers(
                add_gzip_header,
                body_len,
                language_binding_name,
                debug_view_tag,
            ),
        }
    }

    /// Verifies if current request is for a deletion-request ping.
    pub fn is_deletion_request(&self) -> bool {
        // The path format should be `/submit/<app_id>/<ping_name>/<schema_version/<doc_id>`
        self.path
            .split('/')
            .nth(3)
            .map(|url| url == "deletion-request")
            .unwrap_or(false)
    }

    /// Decompress and pretty-format the ping payload
    ///
    /// Should be used for logging when required.
    /// This decompresses the payload in memory.
    pub fn pretty_body(&self) -> Option<String> {
        let mut gz = GzDecoder::new(&self.body[..]);
        let mut s = String::with_capacity(self.body.len());

        gz.read_to_string(&mut s)
            .ok()
            .map(|_| &s[..])
            .or_else(|| std::str::from_utf8(&self.body).ok())
            .and_then(|payload| serde_json::from_str::<JsonValue>(payload).ok())
            .and_then(|json| serde_json::to_string_pretty(&json).ok())
    }

    /// Attempt to gzip the provided ping content.
    fn gzip_content(path: &str, content: &[u8]) -> Option<Vec<u8>> {
        let mut gzipper = GzEncoder::new(Vec::new(), Compression::default());

        // Attempt to add the content to the gzipper.
        if let Err(e) = gzipper.write_all(content) {
            log::error!("Failed to write to the gzipper: {} - {:?}", path, e);
            return None;
        }

        gzipper.finish().ok()
    }

    /// Creates the default request headers.
    fn create_request_headers(
        is_gzipped: bool,
        body_len: usize,
        language_binding_name: &str,
        debug_view_tag: Option<&String>,
    ) -> HashMap<&'static str, String> {
        let mut headers = HashMap::new();
        headers.insert("Date", create_date_header_value(Utc::now()));
        headers.insert(
            "User-Agent",
            create_user_agent_header_value(crate::GLEAN_VERSION, language_binding_name, system::OS),
        );
        headers.insert("X-Client-Type", "Glean".to_string());
        headers.insert(
            "Content-Type",
            "application/json; charset=utf-8".to_string(),
        );
        headers.insert("Content-Length", body_len.to_string());
        if is_gzipped {
            headers.insert("Content-Encoding", "gzip".to_string());
        }
        headers.insert("X-Client-Version", crate::GLEAN_VERSION.to_string());
        if let Some(tag) = debug_view_tag {
            headers.insert("X-Debug-ID", tag.clone());
        }
        headers
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::offset::TimeZone;

    #[test]
    fn test_date_header_resolution() {
        let date: DateTime<Utc> = Utc.ymd(2018, 2, 25).and_hms(11, 10, 37);
        let test_value = create_date_header_value(date);
        assert_eq!("Sun, 25 Feb 2018 11:10:37 GMT", test_value);
    }

    #[test]
    fn test_user_agent_header_resolution() {
        let test_value = create_user_agent_header_value("0.0.0", "Rust", "Windows");
        assert_eq!("Glean/0.0.0 (Rust on Windows)", test_value);
    }
}
