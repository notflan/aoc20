CARGO_FEATURE_FLAGS?=

RUSTFLAGS?=-C target-cpu=native

.PHONY: all
.NOTPARALLEL: all
all:
	$(MAKE) part1
	$(MAKE) part2

.NOTPARALLEL: part1
part1: $(wildcard src/*.rs)
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --release $(addprefix --features ,$(CARGO_FEATURE_FLAGS))
	mv -f target/release/day7 $@
	strip $@
	
.NOTPARALLEL: part2
part2: $(wildcard src/*.rs)
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --release --features $@ $(addprefix --features ,$(CARGO_FEATURE_FLAGS))
	mv -f target/release/day7 $@
	strip $@
	
clean:
	rm -f part{1,2}
