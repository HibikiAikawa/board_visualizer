import asyncio

import board_fetcher
from board_fetcher import run_coincheck

async def main():
    await run_coincheck(10)
    # data_queue = client.run_worker()
    
if __name__ == "__main__":
    asyncio.run(main())