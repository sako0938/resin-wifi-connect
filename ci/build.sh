#!/bin/bash

set -ev

if [ -z "$TARGET" ] || [ -z "$BINARY" ]; then
	echo "Please set value for TARGET and BINARY"
	exit 1
fi

docker build -t $TARGET ci/docker/$TARGET

docker run -it --rm -v ${PWD}:/work $TARGET cargo build --release --target=$TARGET

docker run -it --rm -v ${PWD}:/work $TARGET stripbin target/$TARGET/release/$BINARY
