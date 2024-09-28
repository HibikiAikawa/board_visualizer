#!/bin/bash
ROOT_DIR=$(dirname $(dirname $(realpath $0)))

docker build ${ROOT_DIR}/docker -t board_visualizer
docker run -it -v ${ROOT_DIR}:/work -w /work board_visualizer