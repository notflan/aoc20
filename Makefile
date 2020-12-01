
DAYS= day1

.PHONY: all

day%/part2: day%
	cd $< && $(MAKE)

all: $(addsuffix /part2,$(DAYS))

clean:
	for d in "$(shell find . -type d -name day\*)"; do cd $$d && $(MAKE) clean; done
		
