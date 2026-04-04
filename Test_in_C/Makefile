CFLAGS=-Wall -g
OUTPUT=main
RUNTIME=2000
READERS=2

all: library

compile: main.c
	gcc $(CFLAGS) -o $(OUTPUT) main.c

library: main.c
	gcc -shared -o lib_lock_speed.so -fPIC main.c

clean:
	rm -rf $(OUTPUT)

test:
	./$(OUTPUT) $(READERS) $(RUNTIME)

run:
	python ./graph.py

