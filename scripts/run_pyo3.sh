#!/bin/sh

cd pyo3/math_tools || exit
uv build
uv run python3 main.py
