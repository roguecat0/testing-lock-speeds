CC=gcc
CFLAGS=-Wall -g
OBJ=main.o math_ops.o librust_ffi.a

app: $(OBJ)
	$(CC) $(OBJ) -o app

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f *.o app

run:
	./app
