SRC:= day9.c map.c

OPT_FLAGS?= -march=native -flto \
	 -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

COMMON_FLAGS?=-pipe -O3 -Wall -pedantic -Wextra -Wstrict-aliasing

C_OPT_FLAGS=$(OPT_FLAGS)
CXX_OPT_FLAGS=$(OPT_FLAGS) -felide-constructors -fno-exceptions
LD_OPT_FLAGS?=-O3 -flto

INCLUDE=../common/include

CFLAGS?=$(COMMON_FLAGS) $(C_OPT_FLAGS) --std=gnu11
CXXFLAGS?=$(COMMON_FLAGS) $(CXX_OPT_FLAGS) --std=gnu++20
LDFLAGS?=$(LD_OPT_FLAGS)

CFLAGS+=-I$(INCLUDE)

.PHONY: all
all: part1 part2

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "$$line," >> $@; \
	done < $<
	
part1: $(SRC) | input.h
	$(CC) $(SRC) $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

part2: $(SRC) | input.h
	$(CC) $(SRC) -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

part1-test: $(SRC) | input-test.h
	$(CC) $(SRC) -DTEST $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

clean:
	rm -f part{1,2}{,-test}
	rm -f input{,-test}.h
