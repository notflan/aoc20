

.PHONY: all
all: target/release/day2

target/release/day2:
	cargo build --release
	ln -sf $@ part2
	strip $@

clean:
	rm -f target/release/day2
	rm -f part2
