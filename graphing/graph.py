import matplotlib.pyplot as plt
import numpy as np
import ctypes

import os

print(os.getcwd())
lib1 = ctypes.CDLL("../Test_in_C/lib_lock_speed.so")
lib1.mutex_reads.argtypes = [ctypes.c_uint16, ctypes.c_uint32]
lib1.mutex_reads.restype = ctypes.c_float

lib2 = ctypes.CDLL("../target/release/librs_locks.so")
lib2.mutex_bench.argtypes = [ctypes.c_uint16, ctypes.c_uint32]
lib2.mutex_bench.restype = ctypes.c_float

x = list(range(1, 11))
results1 = [lib1.mutex_reads(i, 3000) for i in x]
results2 = [lib2.mutex_bench(i, 3000) for i in x]
results = [results1, results2]

plt.plot(x, results1, marker="o", linestyle="-")
plt.plot(x, results2, marker="x", linestyle="-")

plt.xlabel("Threads")
plt.ylabel("Reads/µS")


y_max = max([max(results1), max(results2)])

y_start = 0
y_end = 5 * int(y_max // 5 + 1)

plt.yticks(np.arange(0, 105 + 5, 5))


plt.grid(axis="y", linestyle="--", alpha=0.7)

plt.tight_layout()

plt.savefig("plot.png")
