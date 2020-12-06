COMMON_OPT_FLAGS?= -O3 -march=native -pipe -flto \
	 -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

C_OPT_FLAGS?=
CXX_OPT_FLAGS?= -felide-constructors
LD_OPT_FLAGS?=-O3 -flto

INCLUDE=../common/include

COMMON_FLAGS=-Wall -pedantic $(COMMON_OPT_FLAGS)

CFLAGS?=$(COMMMON_FLAGS) --std=gnu11 $(C_OPT_FLAGS)
CXXFLAGS?=$(COMMON_FLAGS) --std=gnu++20 $(CXX_OPT_FLAGS)
LDFLAGS?=$(LD_OPT_FLAGS)


CXXFLAGS+= -fno-exceptions -I$(INCLUDE)
CFLAGS+= -I$(INCLUDE)

.PHONY: all
all: part1 part2

part1: day3.cpp
	$(CXX) $< $(CXXFLAGS) -o $@ $(LDFLAGS)
	strip $@

part2: day3.cpp
	$(CXX) $< -DPART2 $(CXXFLAGS) -o $@ $(LDFLAGS)
	strip $@

clean:
	rm -f part{1,2}
