#!/bin/bash

docker build -t $1 ./ci/docker/$1

docker run -it --rm -v ${PWD}:/work $1 cargo build --release --target=$1
