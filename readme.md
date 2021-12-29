Software Testing finals project.

# Building the app

You need cargo (the rust languages build tool) installed.

Make sure the `RUSTFLAGS` env variable is empty (or does not contain `--cfg blackbox_tests`).

Simply run:
```sh
git clone https://github.com/insomnimus/testing-finals-1
cd testing-finals-1
git checkout main # Necessary if your git is old
cargo build --release --locked
```

# To run the tests
If you have docker installed, all you need is build and run the image:

```sh
docker build -t tests .
docker run -it tests
```

If you don't have docker, you will need to install the go programming language (1.16 or above).
`cargo` is also required.

```sh
# on powershell:
# $env:RUSTFLAGS = "--cfg blackbox_tests"
# On bash/zsh:
export RUSTFLAGS="--cfg blackbox_tests"

# Build and run the test suite.
# (the environment variable ust be set as shown above)
cargo test

# To run blackbox tests, build the app first.
cargo build

# Build the testing script
cd blackbox-tests
go build -o ./tests .
# on windows the same line is:
# go build -o .\tests.exe .

# Run it
./tests ../target/debug/chat
# Or on windows:
# .\tests.exe ..\target\debug\chat.exe
```
