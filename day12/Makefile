
OPT_FLAGS?= -march=native -flto \
	 -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

CFLAGS+= -Wall -pedantic --std=gnu11 -Wstrict-aliasing
CFLAGS+= $(OPT_FLAGS)
CFLAGS+= -O3 -pipe
LDFLAGS?= -O3 -flto

INCLUDE?=../common/include
CFLAGS+= -I$(INCLUDE)

STRIP=strip

.PHONY: all
all: part1 part2

.PHONY: test
test: part1-test part2-test

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "\"$$line\"," >> $@; \
	done < $<	

part1: day12.c | input.h
	$(CC) $< $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

part1-test: day12.c | input-test.h
	$(CC) $< -DTEST $(CFLAGS) -o $@ $(LDFLAGS)

part2: day12.c | input.h
	$(CC) $< -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

part2-test: day12.c | input-test.h
	$(CC) $< -DTEST -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
clean:
	rm -f input{,-test}.h
	rm -f part{1,2}{,-test}
