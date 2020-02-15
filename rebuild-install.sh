#!/usr/bin/env bash
set -ex

pip install -U setuptools wheel setuptools-rust twine

case "$1" in
    debug)
        pip install -U --force-reinstall -e .
    ;;
    *)
        pip install -U --force-reinstall .
    ;;
esac
