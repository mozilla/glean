/// Sanitizes the application id, generating a pipeline-friendly string that replaces
/// non alphanumeric characters with dashes.
pub fn sanitize_application_id(application_id: &str) -> String {
    let mut last_dash = false;
    application_id
        .chars()
        .filter_map(|x| match x {
            'A'...'Z' | 'a'...'z' | '0'...'9' => {
                last_dash = false;
                Some(x)
            }
            _ => {
                let result = if last_dash { None } else { Some('-') };
                last_dash = true;
                result
            }
        })
        .collect()
}

#[test]
fn test_sanitize_application_id() {
    assert_eq!(
        "org-mozilla-test-app",
        sanitize_application_id("org.mozilla.test-app")
    );
    assert_eq!(
        "org-mozilla-test-app",
        sanitize_application_id("org.mozilla..test---app")
    );
    assert_eq!(
        "org-mozilla-test-app",
        sanitize_application_id("org-mozilla-test-app")
    );
}
