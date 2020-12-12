SRC:= day9.c map.c

OPT_FLAGS?= -march=native -flto \
	 -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	 -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	 -fno-stack-check -fno-strict-aliasing 

COMMON_FLAGS?=-pipe -O3 -Wall -pedantic -Wextra -Wstrict-aliasing -Wcast-align

C_OPT_FLAGS=$(OPT_FLAGS)
CXX_OPT_FLAGS=$(OPT_FLAGS) -felide-constructors -fno-exceptions
LD_OPT_FLAGS?=-O3 -flto

INCLUDE?=../common/include

CFLAGS?=$(COMMON_FLAGS) $(C_OPT_FLAGS) --std=gnu11
CXXFLAGS?=$(COMMON_FLAGS) $(CXX_OPT_FLAGS) --std=gnu++20
LDFLAGS?=$(LD_OPT_FLAGS)

CFLAGS+=-I$(INCLUDE)

STRIP=strip

DEFINES?=-DCATERPILLAR

CFLAGS+= $(DEFINES)
CXXFLAGS+= $(DEFINES)

# Number of iterations to profile program
PROF_ITERATIONS?=40
# Set concurrent profiling runs to be number of *physical* cpu cores
PROF_CONCURRENT?=$(shell grep ^cpu\\scores /proc/cpuinfo | uniq |  awk '{print $4}') 

.PHONY: all
all: part1 part2

.PHONY: test
test: part1-test part2-test

inpu%.h: inpu%
	@rm -f $@
	while read line; do \
		echo "$$line," >> $@; \
	done < $<
	
part1: $(SRC) | input.h
	$(CC) $(SRC) $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

part2: $(SRC) | input.h
	rm -f *.gcda
	$(CC) $(SRC) -DPART2 $(CFLAGS) -fprofile-generate -o $@ $(LDFLAGS)
	for i in {0..$(PROF_ITERATIONS)}; do \
		printf "\rIteration $$i / $(PROF_ITERATIONS)"; \
		for j in {0..$(PROF_CONCURRENT)}; do \
			./$@ >> /dev/null 2>&1 & \
		done; \
		wait; \
	done
	@echo " DONE"
	rm $@
	$(CC) $(SRC) -DPART2 $(CFLAGS) -fprofile-use -fprofile-correction -o $@ -lgcov $(LDFLAGS) -fprofile-use -fprofile-correction
	$(STRIP) $@

part1-test: $(SRC) | input-test.h
	$(CC) $(SRC) -DTEST $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

part2-test: $(SRC) | input-test.h
	$(CC) $(SRC) -DTEST -DPART2 $(CFLAGS) -o $@ $(LDFLAGS)
	$(STRIP) $@

clean:
	rm -f part{1,2}{,-test}
	rm -f input{,-test}.h
	rm -f *.gcda
