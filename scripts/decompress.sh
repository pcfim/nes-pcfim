#!/bin/bash

if [ -e "build/files" ]; then
    echo "Files already decompressed"
else
    echo "Decompressing files"
	cat static/files.part_* > build/files.gz
	tar -xzf build/files.gz -C build/
fi