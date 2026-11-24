#bin/bash
cargo run -- build $PWD/src/main.ps
cc $PWD/main.o -o program
./program
echo $?
sha256sum ./program
rm $PWD/program $PWD/main.o
