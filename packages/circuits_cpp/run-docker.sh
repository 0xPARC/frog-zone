#!/bin/sh

set -e

VERSION=v5

if [ "$SUDO" = 1 ]; then
	DOCKER="sudo docker"
else
	DOCKER="docker"
fi

func="$1"

# $DOCKER run --rm -it --entrypoint /bin/bash \
#	--mount type=bind,source="$(pwd)/transpiler",target=/usr/src/fhe/transpiler \
$DOCKER run --platform linux/amd64 --rm -it --entrypoint /usr/src/fhe/compile-frogzone.sh \
	--mount type=bind,source="$(pwd)/src",target=/projects/frogzone \
	--mount type=bind,source="$(pwd)/compile-frogzone.sh",target=/usr/src/fhe/compile-frogzone.sh \
	-e func="${func}" \
	"ed255/phantom-zone:$VERSION"
