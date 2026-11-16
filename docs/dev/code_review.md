# Code Review

Code review is our basic mechanism to share knowledge of the code base.
It should be used to validate the design and implementation of patches.

Every change going into the Glean SDK repository should be reviewed by at least one other Glean maintainer.
Exceptions can apply (see below).

Reviewers are automatically assigned to new pull requests using GitHub's [code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners).
Reviewers are picked from the Glean team in a round-robin fashion.
This can be configured in the [team settings](https://github.com/orgs/mozilla/teams/glean/edit/review_assignment).

Individuals on the Glean team, explicitly tagged for review or feedback, are expected to respond within two business days.

If you don’t hear back within the expected time, please ping the individual in the pull request or over one of the [monitored communication channels](../book/index.html#contact).
If the individual is not available, please kindly reach out to their manager.

## What do reviewers look for?

See also the Firefox [Code Review FAQ](https://firefox-source-docs.mozilla.org/contributing/Code_Review_FAQ.html).

## Exceptions for when code review might be skipped

* After a [release](cut-a-new-release.md) merging back the release changelog into `main` does not require additional
  review.
