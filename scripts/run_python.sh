#!/bin/sh

cd bindings/python || exit
make build-so

python3 main.py
