# Run glean-py Windows tests in Docker

This should be equivalent to what is running on CI.

## Build the image

```
docker build -t gleanwin .
```

## Run a build & test

```
docker run -t -i --rm gleanwin
```

Inside the container:

```
~/build.sh
```

This builds the target, installs it for use in the wine environment and runs the Python tests.

To rerun the tests:

```
$WINPYTHON -m pytest -s glean-core/python/tests
```
