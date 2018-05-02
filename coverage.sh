#!/bin/bash
#
# coverage.sh
#
# Generate code coverage reports using kcov. Requires `kcov` to be in `$PATH`.
# This script is written to be used with debian-likes that have `apt` available.
#
# Note: as Rust does not output separate binaries for doc-tests kcov cannot
# generate coverage data for doc-tests. All doc-tests presumably cover the piece
# of code they're tied to though.
#
# Usage:
#
# First build the application and run tests on it
#     
#     $ cargo build
#     $ RUSTFLAGS='-C link-dead-code' cargo test
#
# (We link dead code to generate proper 0% coverage reports for it.)
#
# Then run
#
#     $ ./coverage.sh run
#
# And results should appear inside ./target/cov/ which includes kcov
# formats, such as cobertura compatible files.
#
# You can also install kcov on a debian-like system somewhat automatically by
# using
#
#     $ ./coverage.sh install
#
# This requires sudo rights. Pass in `--yes` to skip confirmation.
#
# Thanks Razican at https://medium.com/@Razican/continuous-integration-and-code-coverage-report-for-a-rust-project-5dfd4d68fbe5
# for the reference on how to use kcov with Rust projects.

set -e

CMD=$1
SAYYES=$2

# If we want to install kcov
if [[ "$CMD" == "install" ]]; then
    echo "Installing kcov and dependencies"

    is_installed=$(which kcov)

    if [[ "$is_installed" != "" ]]; then
        echo "kcov is already installed at $is_installed"
        exit 0
    fi

    echo "This script is intended for debian-like distros"
    echo "This script will attempt to install the following dependencies:"
    echo "    libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev"

    # if we dont force then ask for confirmation
    if [[ "$SAYYES" != "--yes" ]]; then
        echo "Are you sure you want to proceed? [y/n]"
        read agree

        if [[ "$agree" != "y" ]]; then
            echo "Cancelled"
            exit 1
        fi
    else
        echo "Installation requested with skip confirmation"
    fi

    echo "Installing dependencies ..."
    sudo apt-get -yqq update
    sudo apt-get -yqq install libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev
    echo "Downloading kcov source ..."
    curl -O --location --silent https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
    tar xzf master.tar.gz
    cd kcov-master
    mkdir build
    cd build
    cmake ..
    make
    sudo make install
    cd ../..
    rm -rf kcov-master

    exit 0
elif [[ "$CMD" == "run" ]]; then
    # first we clean up
    rm -rf ./target/cov

    # sometimes kcov does not seem to generate all reports properly, need to
    # investigate if this is because kcov, rust, or our code does something
    # wrong
    #
    # hence we do it until a merged dir is available
    while [[ ! -d "./target/cov/merged" ]]; do
        combined_src=""

        # wppr bins
        for file in ./target/debug/wppr-*[^\.d]; do
            fbasename=$(basename $file)
            mkdir -p ./target/cov/$fbasename
            kcov --exclude-pattern=/.cargo,/usr/lib,tests/,main.rs --include-pattern=src/ --verify "./target/cov/$fbasename" "$file"
            combined_src="$combined_src ./target/cov/$fbasename"
        done

        # wordpress_test bins
        for file in ./target/debug/wordpress_test-*[^\.d]; do
            fbasename=$(basename $file)
            mkdir -p ./target/cov/$fbasename
            kcov --exclude-pattern=/.cargo,/usr/lib,tests/,main.rs --include-pattern=src/ --verify "./target/cov/$fbasename" "$file"
            combined_src="$combined_src ./target/cov/$fbasename"
        done

        # merge tests
        kcov --merge ./target/cov/merged $combined_src

        # remove intermediates
        rm -rf $combined_src
    done
else
    echo "Usage:"
    echo
    echo "    ./coverage.sh run"
    echo "    To generate coverage data"
    echo 
    echo "    ./coverage.sh install [system]"
    echo "    To install kcov on a debian-like system (requires sudo)"

    exit 1
fi