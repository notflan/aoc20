
OPT_FLAGS?= -O3 -march=native -pipe -flto \
	 -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

CFLAGS?=$(OPT_FLAGS)
LDFLAGS?=-O3 -flto

INCLUDE?=../common/include

CFLAGS+= -Wall -Wextra -Wstrict-aliasing -pedantic --std=gnu11 -I$(INCLUDE)

.PHONY: all
all: part1 part2 

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "\"$$line\"," >> $@; \
	done < $<

part1: day6.c | input.h
	$(CC) $< $(CFLAGS) -o $@ $(LDFLAGS)

part1-test: day6.c | input-test.h
	$(CC) $< -DTEST $(CFLAGS) -o $@ $(LDFLAGS)
	
part2: day6.c | input.h
	$(CC) $< -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)

part2-test: day6.c | input-test.h
	$(CC) $< -DTEST -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
clean:
	rm -f part{1,2}{,-test}
	rm -f input.h

