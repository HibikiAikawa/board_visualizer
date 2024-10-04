#!/bin/bash


source ~/docker/poetry_setting.sh
source ~/.cargo/env

cd /work/board_visualizer
poetry install

cd board_fetcher
poetry -C ../ run maturin develop

cd ../
poetry run python board_visualizer/src/new_listed_crypto_fetcher.py