:: Build all docs with one command, on Windows.
:: Documentation will be placed in `build/docs`.
:: This behaves the same as build-rust-docs.sh.

:: Note: there's no way to do "set -e" easily in
:: Windows batch file, other than this:
:: https://stackoverflow.com/a/13863374/261698
:: I'm ignoring this for the moment, as I'm the
:: only consumer for now :-)

:: Set the docs location.
set "docs_location=build\docs"

:: Set the crate name.
set "crate_name=glean_core"

:: Switch to the 'docs' subdirectory, build using
:: mdbook and get back to the current directory.
pushd docs && mdbook build && popd

cargo doc --no-deps

if exist %docs_location% rmdir /S /Q %docs_location%
mkdir %docs_location%
echo "<meta http-equiv=refresh content=0;url=book/index.html>" > %docs_location%\index.html

mkdir %docs_location%\book
xcopy /K /E docs\book\ %docs_location%\book

mkdir %docs_location%\docs
xcopy /K /E target\doc\. %docs_location%\docs
echo "<meta http-equiv=refresh content=0;url=%crate_name%/index.html>\n" > %docs_location%\docs\index.html
