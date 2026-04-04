import matplotlib.pyplot as plt
import numpy as np
import ctypes

import os

print(os.getcwd())
lib = ctypes.CDLL("../Test_in_C/lib_lock_speed.so")
lib.mutex_reads.argtypes = [ctypes.c_uint16, ctypes.c_uint32]
lib.mutex_reads.restype = ctypes.c_float

lib = ctypes.CDLL("./liblock_bencher.so")
lib.bench_result.argtypes = [ctypes.c_uint16, ctypes.c_uint32]
lib.bench_result.restype = ctypes.c_float

x = list(range(1, 3))
results1 = [lib.mutex_reads(i, 3000) for i in x]
results2 = [lib.bench_result(i, 3000) for i in x]
results = [results1, results2]

plt.plot(x, results, marker="o", linestyle="-")

plt.xlabel("Threads")
plt.ylabel("Reads/µS")


y_max = max([max(results1), max(results2)])

y_start = 0
y_end = 5 * int(y_max // 5 + 1)

plt.yticks(np.arange(0, y_max + 5, 5))


plt.grid(axis="y", linestyle="--", alpha=0.7)

plt.tight_layout()

plt.savefig("plot.png")
