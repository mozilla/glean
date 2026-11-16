# Reviewing Dependabot Pull Requests

We use [Dependabot](https://docs.github.com/en/code-security/tutorials/secure-your-dependencies/dependabot-quickstart)
for automated updates of dependencies in the Glean SDK repository.

The configuration lives in
[`.github/dependabot.yml`](https://github.com/mozilla/glean/blob/main/.github/dependabot.yml).

It currently runs for Rust dependencies, Python dependencies as well as external actions used in our CI configuration.

Dependabot will open pull requests with the updated versions, which automatically triggers CI.

## For reviewers

Dependency updates usually fall into one of these categories:

1. Regular updates to dependencies of the Glean SDK
2. Regular updates to _development_ dependencies (those used in the build system, in build tooling,
   for documentation generation or similar)
3. Security updates for dependencies

### General review

Dependency updates require review by any member of the Glean SDK team.
A reviewer is picked automatically.

As a reviewer you should focus on:

* Is CI green? If not, identify the issue.
  * If CI is failing due to intermittent issues, rerun the CI task. If it keeps failing, file a bug.
  * If CI is failing, but the failure is already fixed in `main`, rebase the PR. Dependabot can do that for you, leave a comment on the PR: `@dependabot rebase`.
  * If CI is failing due to a change in the dependency you can reject the pull request.
    You should file a bug to eventually address the necessary update in the future.
* Check the release notes and commits as listed in the PR description. If anything sticks out consider reviewing the
  changes in that dependency more closely.
  * Look out for dependencies dropping support for older versions of the language (e.g. dropping Python 3.9). These
    might not be caught by our CI immediately.

If you are unsure about a dependency update at any point, ask a team mate.

If CI is green and you have reviewed the PR merge it.
If you decide to dismiss the PR for some reason, state so in a comment and close the PR (see below for possible reasons
for dismissing a PR).

If you dismiss the PR you can instruct Dependabot to ignore this dependency in the future.
See the "Dependabot commands and options" listing in the PR description.

### Category 1: Dependencies of the Glean SDK

Follow the general review guidelines.

Consider the impact the dependency update might have to downstream users of the Glean SDK.
Especially major version updates might force downstream users to update as well.
If the current version works for us we might consider staying on the older version.

This is especially true for Rust dependencies.
These need to be kept in sync with what is vendored into [Firefox](https://github.com/mozilla-firefox/firefox/tree/main/third_party/rust).
See also [Dependency vetting](core/dependency-vetting.html).

### Category 2: Development dependencies

Follow the general review guidelines.

Any issues with development dependencies _should_ be caught by our CI.
If in doubt run the necessary build or check locally using the updated dependency.

### Category 3: Security Updates

These will also be listed in the [repository security page](https://github.com/mozilla/glean/security).
Security updates require immediate attention.

As a reviewer you should:

* assess whether the flagged security issue applies to us
* review the pull request

If you determine that the security issue does apply to the use of this dependency in the Glean SDK you must ensure this
PR can be merged.
This might require additional work if the update breaks the use in the Glean SDK, as indicated by failing tests or CI
checks.

If you determine that the security issue does not apply to the use of this dependency in the Glean SDK you have two
options:

* Review the PR as usual and merge when green.
* Close the PR and dismiss the security issue on the [security page](https://github.com/mozilla/glean/security). Include
  a reason for the dismissal.
