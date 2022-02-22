#!/bin/sh
# TODO: setup cross-compile
git clone git://git.musl-libc.org/musl . -b $VERSION --depth 1

./configure --prefix=/System --libdir=/System/Library/$TARGET_ARCH --syslibdir=/System/Library/$TARGET_ARCH --includedir=/System/SDK/include --bindir=/System/SDK/bin
make -j
make install
