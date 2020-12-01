
DAYS= day1

.PHONY: all

day%/part2: day%
	cd $< && $(MAKE)

all: $(addsuffix /part2,$(DAYS))
		
