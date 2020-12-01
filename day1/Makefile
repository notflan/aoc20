
OPT_FLAGS?= -march=native -fgraphite -fopenmp -floop-parallelize-all -ftree-parallelize-loops=4 \
	    -floop-interchange -ftree-loop-distribution -floop-strip-mine -floop-block \
	    -fno-stack-check -fno-strict-aliasing

.PHONY: all
all: part1 part2

.PHONY: big
big: part1-big part2-big

input.h: input
	@rm -f $@
	while read line; do \
		echo "$$line," >> $@; \
	done < $<

input-big.h: input-big
	@rm -f $@
	while read line; do \
		echo "$${line}ul," >> $@; \
	done < $<


part1: input.h
	gcc day1.c -O3 -flto $(OPT_FLAGS) -o $@ -O3 -flto
	strip $@

part2: input.h
	gcc day1.c -DPART2 -O3 -flto $(OPT_FLAGS) -o $@ -O3 -flto
	strip $@

part1-big: input-big.h
	gcc day1.c -DBIG -O3 -flto $(OPT_FLAGS) -o $@ -O3 -flto
	strip $@

part2-big: input-big.h
	gcc day1.c -DBIG -DPART2 -O3 -flto $(OPT_FLAGS) -o $@ -O3 -flto
	strip $@

clean:
	rm -f part{1,2}{,-big}
	rm -f input{,-big}.h
