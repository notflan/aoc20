COMMON_OPT_FLAGS?= -DFROM_MAIN -march=native -flto \
	 -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

C_OPT_FLAGS?=
CXX_OPT_FLAGS?= -felide-constructors -fno-exceptions
LD_OPT_FLAGS?=-O3 -flto

INCLUDE=$(shell pwd)/common/include

COMMON_FLAGS=-pipe -O3 -Wall -Wextra -Wstrict-aliasing -pedantic $(COMMON_OPT_FLAGS)

RUSTFLAGS+=-C target-cpu=native
CFLAGS+=$(COMMON_FLAGS) --std=gnu11 $(C_OPT_FLAGS)
CXXFLAGS+=$(COMMON_FLAGS) --std=gnu++20 $(CXX_OPT_FLAGS)
LDFLAGS+=$(LD_OPT_FLAGS)

DAYS= $(wildcard day*)

ENV= CFLAGS="$(CFLAGS)" CXXFLAGS="$(CXXFLAGS)" LDFLAGS="$(LDFLAGS)" INCLUDE="$(INCLUDE)" RUSTFLAGS="$(RUSTFLAGS)"

.PHONY: all

all: $(addsuffix /part2,$(DAYS))

day%/part2: day%
	cd $< && $(ENV) $(MAKE)

clean:
	for d in $(DAYS); do pushd $$d && $(MAKE) clean && popd; done
		
