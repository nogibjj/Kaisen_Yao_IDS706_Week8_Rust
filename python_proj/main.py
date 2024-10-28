"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query
import time
import psutil
from contextlib import redirect_stdout

# Extract
print("Extracting data...")
extract()


start_time = time.perf_counter()
memory_before = psutil.virtual_memory().used / (1024.0 * 1024.0)

# Transform and load
print("Transforming data...")
load()

# Query
print("Querying data...")
query()

end_time = time.perf_counter()
elapsed_time_micros = (end_time - start_time) * 1e6
memory_after = psutil.virtual_memory().used / (1024.0 * 1024.0)
memory_used = memory_after - memory_before

with open("../python_performance.md", "w") as f:
    with redirect_stdout(f):
        print(f"Elapsed time: {elapsed_time_micros:.2f} microseconds")
        print(f"Memory used: {memory_used:.2f} MB")
