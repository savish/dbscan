#!/bin/bash

grep -v '\/\/!' ./src/lib.rs > temp && mv temp ./src/lib.rs
sed 's|^|//! &|' ./README.md > temp
cat temp ./src/lib.rs > temp2
mv temp2 ./src/lib.rs
rm temp