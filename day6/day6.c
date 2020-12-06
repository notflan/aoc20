
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#if defined(TEST) && !defined(DEBUG)
#define DEBUG
#endif

static const char* const answers[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};

#define  answers_sz (sizeof(answers) / sizeof(char*))

#define NUM_QUESTIONS 26

typedef struct answer {
	bool table[NUM_QUESTIONS];
} answers_t;

inline static char assert_in_bound(char i)
{
	register int x=(int)i;
	if(x<0 || x>=NUM_QUESTIONS) {
		fprintf(stderr, "Fatal error: char '%c' (%d) is not in range 0..%d\n", i, x, NUM_QUESTIONS);
		abort();
	}
	return i;
}

static void populate(const char* from, answers_t * restrict ans) //wtf is this syntax? `bool* restrict a` -> `bool a[restrict N]`????
{
	while(*from)
		ans->table[(int)assert_in_bound((*from++)-'a')] = true;
}

static size_t count_ans(const answers_t* restrict ans) 
{
	register size_t j=0;
	for(register size_t i=0;i<NUM_QUESTIONS;i++)
		j+= ans->table[i] ? 1 : 0;
	return j;
}

inline static size_t reset(bool* restrict pop, answers_t * restrict ans, size_t* restrict group_count)
{
	register size_t fullcnt=0;
	if(*pop) {
		fullcnt = (*group_count = count_ans(ans));
#ifdef DEBUG
		fprintf(stderr, "Last group: %lu (fcnt %lu)\n" , *group_count, fullcnt);
#endif
	}
	*pop = false;
	memset(ans,0, sizeof(answers_t));
	return fullcnt;
}

int main()
{
	answers_t answered = {0};
	size_t group_counts[answers_sz+1] = {0};
	size_t fullcnt=0;
	bool pop=false;

	for(size_t i=0,j=0;i<answers_sz;i++)
	{
		const char* current = answers[i];
		if(*current) {
			populate(current, &answered);
			pop=true;
		} else {
			fullcnt += reset(&pop, &answered, &group_counts[j++]);
		}

	}
	fullcnt+= reset(&pop, &answered, group_counts +answers_sz);
	printf("%lu\n", fullcnt);

	return 0;
}
