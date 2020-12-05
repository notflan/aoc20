#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

#define _cold __attribute__((cold, noinline))
#define noreturn __attribute__((noreturn)) void

#ifdef DEBUG
#define dlog(...) fprintf(stderr, "[debug]" __VA_ARGS__)
#else
inline static void do_nothing(int _n, ...) {}
#define dlog(...) do { if(0) { do_nothing(0 __VA_OPT__(,) __VA_ARGS__); } } while(0)
#endif

noreturn static _cold panic(const char* msg)
{
	fputs(msg, stderr);
	fputc('\n', stderr);
	abort();
}

typedef struct sloc {
	int row;
	int column;

	int id;
} seat_t;

static const char* input[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};

#define input_sz (sizeof(input) / sizeof(char*))


#define ROW_MAX 128
#define COL_MAX 8

inline static void seat_calc_id(seat_t* restrict s)
{
	s->id = (s->row * 8) + s->column;
}


static int sbsearch(const char* direct, size_t len, int max)
{
	int min = 0;
#define diff (1 + (max - min))
	for(size_t i=0;i<len;i++)
	{
		dlog("%c: min = %d, max = %d\n", direct[i], min, max);
		switch(direct[i])
		{
		case 0: 
			panic("string overflow");
		// lower half
		case 'F':
		case 'L':
			max -= diff / 2;
			if(diff==1) return min;
			break;
		// upper half
		case 'R':
		case 'B':
			min += diff / 2;
			if(diff==1) return max;
			break;
		}
	}
	panic("bsearch failed");
#undef diff
}

static seat_t pbsearch(const char* str)
{
	seat_t seat = {
		.row = (int)sbsearch(str, 7, ROW_MAX-1),
		.column = (int)sbsearch(str+7, 3, COL_MAX-1),
	};
	seat_calc_id(&seat);
	return seat;
}

#ifdef PART2
static int _seat_cmp(const void* ptr, const void* ptr2)
{
	const seat_t *s1 = ptr;
	const seat_t *s2 = ptr2;
	return    s1->id < s2->id ? -1 
		: s1->id > s2->id ? 1
		: 0;
}
#endif


int main()
{
	int max=0;
	seat_t seats[input_sz];
	for(size_t i=0;i<input_sz;i++)
	{
		dlog("Testing %s\n", input[i]);
		seat_t seat = seats[i] = pbsearch(input[i]);
		int id = seat.id;
		dlog(" -> { .row = %d, .column = %d, .id = %d }\n", seat.row, seat.column, seat.id);
		if(id>max) max = id;
	}
#ifdef PART2
	qsort(seats, input_sz, sizeof(seat_t), &_seat_cmp);
	for(size_t i=1; i<input_sz;i++)
	{
		seat_t prev = seats[i-1];
		seat_t this = seats[i];
#define diff (this.id - prev.id)
		if(diff == 2)
		{
			printf("%d\n", prev.id+1);
			return 0;
		}
	}
	fprintf(stderr, "Failed to find id\n");
	return 1;
#else
	printf("%d\n", max);
#endif
	return 0;
}
