# Debug an iOS application against different builds of Glean

At times it may be necessary to debug against a local build of Glean or another git fork or branch in order to test new features or specific versions of Glean.

Since Glean is consumed through [Carthage](https://github.com/Carthage/Carthage), this can be as simple as modifying the Cartfile for the consuming application.

## Building against the latest Glean

For consuming the latest version of Glean, the Cartfile contents would include the following line:

```
github "mozilla/glean" "main"
```

This will fetch and compile Glean from the [mozilla/glean GitHub](https://github.com/mozilla/glean/) repository from the "main" branch.

## Building against a specific release of Glean

For consuming a specific version of Glean, you can specify a branch name, tag name, or commit ID, like this:

```
github "mozilla/glean" "v0.0.1"
```

Where `v0.0.1` is a tagged release name, but could also be a branch name or a specific commit ID like `832b222`

If the custom Glean you wish to build from is a different fork on GitHub, you could simply modify the Cartfile to point at your fork like this:

```
github "myGitHubHandle/glean" "myBranchName"
```

Replace the `myGitHubHandle` and `myBranchName` with your GitHub handle and branch name, as appropriate.

## Build from a locally cloned Glean

You can also use Carthage to build from a local clone by replacing the Cartfile line with the following:

```
git "file:///Users/yourname/path/to/glean" "localBranchName"
```

Notice that the initial Carthage command is `git` now instead of `github`, and we need to use a file URL of the path to the locally cloned Glean

## Perform the Carthage update

One last thing not to forget is to run the `carthage update` command in the directory your Cartfile resides in order to fetch and build Glean.

Once that is done, your local application should be building against the version of Glean that you specified in the Cartfile.
