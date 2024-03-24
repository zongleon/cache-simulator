EXES = cyclic_test sawtooth_test compare_cache_size compare_associativity

CFLAGS = -Wall -Werror

.PHONY: all
.PHONY: clean

all: clean cyclic_test sawtooth_test compare_cache_size compare_associativity

cyclic_test:
	clang $(CFLAGS) -o cyclic_test cyclic_test.c

sawtooth_test:
	clang $(CFLAGS) -o sawtooth_test sawtooth_test.c

compare_cache_size:
	clang $(CFLAGS) -o compare_cache_size compare_cache_size.c

compare_associativity:
	clang $(CFLAGS) -o compare_associativity compare_associativity.c

clean:
	-rm $(EXES)

