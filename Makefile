
DAYS= $(wildcard day*)

.PHONY: all

day%/part2: day%
	cd $< && $(MAKE)

all: $(addsuffix /part2,$(DAYS))

clean:
	for d in $(DAYS); do pushd $$d && $(MAKE) clean && popd; done
		
