import matplotlib.pyplot as plt
import numpy as np
import ctypes

lib = ctypes.CDLL("./lib_lock_speed.so")
lib.mutex_reads.argtypes = [ctypes.c_uint16, ctypes.c_uint32]
lib.mutex_reads.restype = ctypes.c_float

x = list(range(1, 11))
results = [lib.mutex_reads(i, 3000) for i in x]

plt.plot(x, results, marker='o', linestyle='-')

plt.xlabel("Threads")
plt.ylabel("Reads/µS")


y_max = max(results)

y_start = 0
y_end = 5 * int(y_max // 5 + 1)

plt.yticks(np.arange(0, max(results) + 5, 5))


plt.grid(axis='y', linestyle='--', alpha=0.7)

plt.tight_layout()

plt.savefig("plot.png")