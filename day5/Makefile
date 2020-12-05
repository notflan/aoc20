SRC:= day5.c

OPT_FLAGS?= -O3 -march=native -pipe -flto \
	 -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

CFLAGS?=$(OPT_FLAGS)
LDFLAGS?=-O3 -flto

CFLAGS+= -Wall -pedantic --std=gnu11 

.PHONY: all
all: part1 part2

.PHONY: test
test: part1-test part2-test

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "\"$$line\"," >> $@; \
	done < $<

part1-test: $(SRC) | input-test.h
	$(CC) $^ -DTEST -DDEBUG $(CFLAGS) -o $@ $(LDFLAGS)

part2-test: $(SRC) | input-test.h
	$(CC) $^ -DTEST -DDEBUG -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)

part1: $(SRC) | input.h
	$(CC) $^ $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

part2: $(SRC) | input.h
	$(CC) $^ -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

clean:
	rm -f part{1,2}{,-test}
	rm -f input{,-test}.h
