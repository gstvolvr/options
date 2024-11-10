import polygon
import logging
from datetime import datetime
from typing import Optional
from multiprocessing import current_process

"""
Help reduce overhead of creating clients for processes
"""
class LazyPolygonClient:
    _instances = {}

    @classmethod
    def get_instance(cls):
        process_name = current_process().name
        if process_name not in cls._instances:
            logging.info(f"Initializing Polygon client in process {process_name}")
            cls._instances[process_name] = polygon.RESTClient()
        return cls._instances[process_name]

def get_last_trading_date(client: polygon.RESTClient) -> Optional[str]:
    previous_closes = client.get_previous_close_agg('SPY')
    for previous_close in previous_closes:
        last_open_date = datetime.fromtimestamp(previous_close.timestamp / 1000)
        return last_open_date.strftime('%Y-%m-%d')