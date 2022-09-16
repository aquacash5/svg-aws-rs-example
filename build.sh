#!/bin/bash

cargo lambda build --release --arm64 --output-format zip
sam deploy