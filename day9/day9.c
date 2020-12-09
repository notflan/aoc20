#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

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
static u64 csum(size_t i, size_t j)
{
	register u64 sum=0;
	for(;i<=j;i++)
		sum += input[i];
	return sum;
}
static int _comp_u64(const void* _i, const void* _j)
{
	const u64* i = _i;
	const u64* j = _j;

	return    *i < *j ? -1
		: *i > *j ? 1 
		: 0;
}
inline static u64 ud_sort(size_t i, size_t j)
{
	u64 slice[input_sz];
	size_t len = j-i;
	assert(len<input_sz);

	memcpy(slice, input+i, sizeof(u64)* len);
	
	qsort(slice, len, sizeof(u64), &_comp_u64);

	return slice[0] + slice[len-1];
}
static u64 find_set(u64 target)
{
	for(register size_t i =0;i<input_sz;i++)
	{
		for(register size_t j=i;j<input_sz;j++)
		{
			u64 sum = csum(i, j);
			if(sum == target) {
				printf("%lu\n", ud_sort(i, j));
				return 0;
			} else if (sum > target) break;
		}
	}
	fprintf(stderr, "No set found\n");
	return 1;
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
