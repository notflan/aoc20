
OPT_FLAGS?= -march=native -flto \
	 -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

INCLUDE?=../common/include

COMMON_FLAGS?=-pipe -O3 -Wall -pedantic -Wextra -Wstrict-aliasing 

C_OPT_FLAGS=$(OPT_FLAGS)
CXX_OPT_FLAGS=$(OPT_FLAGS) -felide-constructors -fno-exceptions
LD_OPT_FLAGS?=-O3 -flto


CFLAGS?=$(COMMON_FLAGS) $(C_OPT_FLAGS) --std=gnu11
CXXFLAG?+=$(COMMON_FLAGS) $(CXX_OPT_FLAGS) --std=gnu++20
LDFLAGS?=$(LD_OPT_FLAGS)

CFLAGS+=-I$(INCLUDE)

.PHONY: all
all: part1 part2

.PHONY: test
test: part1-test part2-test

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "\"$$line\"," >> $@; \
	done < $<

part1: day8.c | input.h
	$(CC) $< $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

part2: day8.c | input.h
	$(CC) $< -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

part1-test: day8.c | input-test.h
	$(CC) $< -DTEST $(CFLAGS) -o $@ $(LDFLAGS)

part2-test: day8.c | input-test.h
	$(CC) $< -DPART2 -DTEST $(CFLAGS) -o $@ $(LDFLAGS)

clean:
	rm -f part{1,2}{,-test}
	rm -f input{,-test}.h
