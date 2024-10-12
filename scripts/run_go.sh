#!/bin/sh

cd bindings/go || exit
go build -o main

./main
