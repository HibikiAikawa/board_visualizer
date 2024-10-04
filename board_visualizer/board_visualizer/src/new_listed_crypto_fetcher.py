import json
import time
import multiprocessing

import asyncio
import requests

import board_fetcher

MAX_BOARD_SIZE = 20
SAVE_TIME_MIN = 60
DIR_PATH = "/work/board_visualizer/data/bybit"

SLEEP_TIME = 30

# 前回の上場リストと比較して新規上場コインを抽出
def extract_new_listed_coin(symbol_list, prev_symbol_list) -> list[str]:
    new_listed_name = []
    for pair in symbol_list:
        flag = False
        for prev_pair in prev_symbol_list:
            if pair["name"] == prev_pair["name"]:
                flag = True
                break
        if not flag:
            new_listed_name.append(pair["name"])
    return new_listed_name

# 同期処理関数に変更
def bybit_live_fetcher(max_board_size, save_time_min, symbol, instrument, dir_path):
    asyncio.run(async_bybit_live_fetcher(max_board_size, save_time_min, symbol, instrument, dir_path))

async def async_bybit_live_fetcher(max_board_size, save_time_min, symbol, instrument, dir_path):
    await board_fetcher.bybit_live_fetcher(
        max_board_size = max_board_size,
        save_time_min = save_time_min,
        symbol = symbol,
        instrument = instrument,
        dir_path = dir_path
    )

def watch_new_listed_coin(api_url, instrument):
    prev_listed_coins = requests.get(api_url).json()["result"]

    while True:
        try:
            time.sleep(SLEEP_TIME)
            listed_coins = requests.get(api_url).json()["result"]
            new_names = extract_new_listed_coin(listed_coins, prev_listed_coins)
            
            if len(new_names) > 0:
                print("new list: ", new_names)
                for symbol in new_names:
                    multiprocessing.Process(
                        target=bybit_live_fetcher,
                        args=(MAX_BOARD_SIZE, SAVE_TIME_MIN, symbol, instrument, DIR_PATH)
                        ).start()
                    time.sleep(1)
            prev_listed_coins = listed_coins
        except Exception as e:
            print(e)
            continue

def main():
    perp_api_url = "https://api.bybit.com/v2/public/symbols"
    spot_api_url = "https://api.bybit.com/spot/v1/symbols"
    multiprocessing.Process(target=watch_new_listed_coin,args=(perp_api_url, "perp")).start()
    multiprocessing.Process(target=watch_new_listed_coin,args=(spot_api_url, "spot")).start()


if __name__ == "__main__":
    main()
