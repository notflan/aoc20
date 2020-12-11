
OPT_FLAGS?= -march=native -flto \
	 -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

CFLAGS+= -Wall -pedantic --std=gnu11 -Wextra -Wstrict-aliasing
CFLAGS+= $(OPT_FLAGS)
CFLAGS+= -O3 -pipe
LDFLAGS?= -O3 -flto

INCLUDE?=../common/include
CFLAGS+= -I$(INCLUDE)

STRIP=strip

.PHONY: all
all: part1 part2

inpu%-sz.h: inpu%
	echo "#define input_sz (`head -n 1 $< | wc -m`)" > $@

inpu%.h: inpu% | input-sz.h
	@rm -f $@
	while read line; do \
		echo "\"$$line\"," >> $@; \
	done < $<
	

part1: day11.c | input.h
	$(CC) $< $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

clean:
	rm -f input{,-sz}.h
	rm -f part{1,2}
