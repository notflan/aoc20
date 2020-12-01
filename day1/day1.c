#include <stddef.h>
#include <stdio.h>

typedef unsigned long u64;

#ifdef BIG
#define TARGET 99920044
#else
#define TARGET 2020
#endif

const u64 input[] = {
#ifdef BIG
#include "input-big.h"
#else
#include "input.h"
#endif
};
const size_t input_sz = sizeof(input)/sizeof(u64);

int main()
{
	for(register u64 i=0;i<input_sz;i++)
		for(register u64 j=i;j<input_sz;j++)
#ifdef PART2
			for(register u64 k=j;k<input_sz;k++)
				if(input[i] + input[j] + input[k] == TARGET)
				{
					printf("%lu\n", input[i] * input[j] * input[k]);
					return 0;
				}
#else
			if (input[i] + input[j] == TARGET)
			{
				printf("%lu\n", input[i] * input[j]);
				return 0;
			}
#endif
	return 1;
}
