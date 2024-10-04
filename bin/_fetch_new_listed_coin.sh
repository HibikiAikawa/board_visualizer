#!/bin/bash


cd /work/board_visualizer
/root/.local/bin/poetry install
/root/.local/bin/poetry run python board_visualizer/src/new_listed_crypto_fetcher.py