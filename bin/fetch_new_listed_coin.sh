#!/bin/bash
ROOT_DIR=$(dirname $(dirname $(realpath $0)))

docker build -f ./docker/Dockerfile . -t board_visualizer
docker run -it --rm -v ${ROOT_DIR}:/work -w /work --name bybit_new_coin_fetcher board_visualizer  \
                                            /bin/bash ./bin/_fetch_new_listed_coin.sh