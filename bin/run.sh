#!/bin/bash
ROOT_DIR=$(dirname $(dirname $(realpath $0)))

docker build -f ./docker/Dockerfile . -t board_visualizer
docker run -it -v ${ROOT_DIR}:/work -w /work board_visualizer