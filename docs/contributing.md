# Contributing to the Glean SDK

Anyone is welcome to help with the Glean SDK project. Feel free to get in touch with other community members on `chat.mozilla.org`
or through issues on GitHub or Bugzilla.

- Matrix: [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org)
- [the Bugzilla issues list][bugzillalist]
- [the GitHub issues list](https://github.com/mozilla/glean/issues)

[bugzillalist]: https://bugzilla.mozilla.org/buglist.cgi?list_id=14844212&resolution=---&classification=Client%20Software&classification=Developer%20Infrastructure&classification=Components&classification=Server%20Software&classification=Other&query_format=advanced&bug_status=UNCONFIRMED&bug_status=NEW&bug_status=ASSIGNED&bug_status=REOPENED&component=Glean%3A%20SDK&product=Data%20Platform%20and%20Tools

Participation in this project is governed by the
[Mozilla Community Participation Guidelines](https://www.mozilla.org/en-US/about/governance/policies/participation/).

## Bug Reports

To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK][newbugzilla].

If you don't have a Bugzilla account, we also accept [issues on GitHub](https://github.com/mozilla/glean/issues/new).

[newbugzilla]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=normal&bug_status=NEW&cf_fission_milestone=---&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox65=---&cf_status_firefox66=---&cf_status_firefox67=---&cf_status_firefox_esr60=---&cf_status_thunderbird_esr60=---&cf_tracking_firefox65=---&cf_tracking_firefox66=---&cf_tracking_firefox67=---&cf_tracking_firefox_esr60=---&cf_tracking_firefox_relnote=---&cf_tracking_thunderbird_esr60=---&product=Data%20Platform%20and%20Tools&component=Glean%3A%20SDK&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&flag_type-203=X&flag_type-37=X&flag_type-41=X&flag_type-607=X&flag_type-721=X&flag_type-737=X&flag_type-787=X&flag_type-799=X&flag_type-800=X&flag_type-803=X&flag_type-835=X&flag_type-846=X&flag_type-855=X&flag_type-864=X&flag_type-916=X&flag_type-929=X&flag_type-930=X&flag_type-935=X&flag_type-936=X&flag_type-937=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=P3&&rep_platform=Unspecified&status_whiteboard=%5Btelemetry%3Aglean-rs%3Am%3F%5D&target_milestone=---&version=unspecified

## Making Code Changes

To work on the code in this repository you will need to be familiar with
the [Rust](https://www.rust-lang.org/) programming language.
You can get a working rust compiler and toolchain via [rustup](https://rustup.rs/).

You can check that everything compiles by running the following from the
root of your checkout:

```
make test-rust
```

If you plan to work on the Android component bindings, you should also review
the instructions for [setting up an Android build environment](dev/android/setup-android-build-environment.md).

To run all Kotlin tests:

```
make test-kotlin
```

or run tests in Android Studio.

To run all Swift tests:

```
make test-swift
```

or run tests in Xcode.

## Sending Pull Requests

Patches should be submitted as [pull requests](https://help.github.com/articles/about-pull-requests/) (PRs).

Before submitting a PR:
- Your code must run and pass all the automated tests before you submit your PR for review.
- "Work in progress" pull requests are allowed to be submitted, but should be clearly labeled as such and should not be merged until all tests pass and the code has been reviewed.
- For changes to Rust code
  - `make test-rust` produces no test failures
  - `make clippy` runs without emitting any warnings or errors.
  - `make rustfmt` does not produce any changes to the code.
- For changes to Kotlin code
  - `make test-kotlin` runs without emitting any warnings or errors.
  - `make ktlint` runs without emitting any warnings.
- For changes to Swift code
  - `make test-swift` (or running tests in Xcode) runs without emitting any warnings or errors.
  - `make swiftlint` runs without emitting any warnings or errors.
- Your patch should include new tests that cover your changes. It is your and your reviewer's responsibility to ensure your patch includes adequate tests.

When submitting a PR:
- You agree to license your code under the project's open source license ([MPL 2.0](https://mozilla.org/MPL/2.0/)).
- Base your branch off the current `main`.
- Add both your code and new tests if relevant.
- Please do not include merge commits in pull requests; include only commits with the new relevant code.

## Code Review

This project is production Mozilla code and subject to our
[engineering practices and quality standards](https://developer.mozilla.org/en-US/docs/Mozilla/Developer_guide/Committing_Rules_and_Responsibilities).
Every patch must be peer reviewed by a member of the Glean core team.

Reviewers are defined in the [CODEOWNERS](https://github.com/mozilla/glean/blob/main/.github/CODEOWNERS) file
and are automatically added for every pull request.
Every pull request needs to be approved by at least one of these people before landing.

The submitter needs to decide on their own discretion whether the changes require a look from more than a single reviewer or any outside developer.
Reviewers can also ask for additional approval from other reviewers.

## Release

See the [Release process](dev/cut-a-new-release.md) on how to release a new version of the Glean SDK.
