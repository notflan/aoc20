#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "map.h"

typedef uint64_t u64;

#ifdef TEST
#define PREAMBLE 5
#else
#define PREAMBLE 25
#endif

#define input_sz (sizeof(input) / sizeof(u64))
const u64 input[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};

#define box(t) (calloc(sizeof(t), 1))

struct pair {
	bool valid;
	u64 num;
	u64 sum[2];
};

inline static void insert_page(map_t* restrict where, u64 key, size_t i)
{
	//if(key > PAGE_SIZE) panic("Data too large, increase page size to fit %lu in a single page", key);
	map_insert(where, key, i);
}

static void calculate_deficits(const u64 pre[static PREAMBLE], map_t * restrict output, u64 target)
{
	for(size_t i=0;i<PREAMBLE;i++)
		if(target > pre[i])
			insert_page(output, target - pre[i], i);
}

#ifdef PART2
static u64 find_set(u64 target)
{
	
}
#endif

int main()
{
	map_t def = map_new();
	struct pair result[input_sz] = {0};
	
	for(register size_t i= PREAMBLE;i<input_sz;i++)
	{
		calculate_deficits(&input[i-PREAMBLE], &def, input[i]);
		
		for(register size_t j=0;j<PREAMBLE;j++)
		{
			result[i].num = input[i];

			size_t idx;
			u64 us = 	  input[(i-PREAMBLE)+j];
			if(!map_get(&def, us, &idx)) continue;
			u64 other = 	  input[(i-PREAMBLE)+idx];

			// Found pair!
			result[i] = (struct pair){.valid = other!=us, .num=input[i], .sum = { us, other }};
		}
		map_clear(&def);
	}
	map_free(def);

	for(register size_t i=PREAMBLE;i<input_sz;i++)
	{
		if(!result[i].valid && !(result[i].sum[0] | result[i].sum[1])) {
#ifdef PART2
			return find_set(result[i].num);
#else
			printf("%lu\n", result[i].num);
			return 0;
#endif
		}
	}
	fprintf(stderr, "Not found\n");
	return 1;
}
