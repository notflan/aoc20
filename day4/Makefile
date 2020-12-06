
CARGO_FEATURE_FLAGS?=

.PHONY: all
.NOTPARALLEL: all
all:
	$(MAKE) part1
	$(MAKE) part2

.NOTPARALLEL: part1
part1: $(wildcard src/*.rs)
	cargo build --release $(addprefix --features ,$(CARGO_FEATURE_FLAGS))
	mv -f target/release/day4 $@
	strip $@
	
.NOTPARALLEL: part2
part2: $(wildcard src/*.rs)
	cargo build --release --features $@ $(addprefix --features ,$(CARGO_FEATURE_FLAGS))
	mv -f target/release/day4 $@
	strip $@
	
clean:
	rm -f part{1,2}
