#!/bin/bash
PICOTCP=~/picotcp
mkdir $PWD/build && PREFIX=$PWD/build make -C $PICOTCP
cp $PWD/build/lib/libpicotcp.a $PWD/target/deps

