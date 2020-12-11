#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <panic.h>
#include <attrs.h>

const char* const input[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};

// input_sz //
#ifdef TEST
#include "input-test-sz.h"
#else
#include "input-sz.h"
#endif

#define INPUT_WIDTH (input_sz-1)
#define INPUT_HEIGHT (sizeof(input)/sizeof(char*)) 

#define box(t) aligned_alloc(_Alignof(t), sizeof(t))

enum gol_state {
	STATE_INVALID=0,
	STATE_FLOOR='.',
	STATE_EMPTY='L',
	STATE_OCCU='#',
};

#define GOL_INDEX(w,h) ((h*INPUT_WIDTH)+w)

typedef struct gol_arena {
	enum gol_state arena[INPUT_HEIGHT*INPUT_WIDTH];
} gol_t;

gol_t* gol_clone(const gol_t* from)
{
	gol_t* out = box(gol_t);
	*out = *from;
	return out;
}

pure gol_t generate_arena()
{
	gol_t gol;
	for(size_t h=0;h<INPUT_HEIGHT;h++)
		for(size_t w=0;w<INPUT_WIDTH;w++)
			if( !(gol.arena[ (h*INPUT_WIDTH)+w ] = (enum gol_state) input[h][w]) ) panic("Invalid `gol_state` char at (%lu x %lu = %lu) '%c' (%d)", w, h, GOL_INDEX(w,h), input[h][w], (int)input[h][w]);

	return gol;
}

static inline pure enum gol_state gol_index(const gol_t* gol, int x, int y)
{
	if (x<0 || x >= (int)INPUT_WIDTH || y<0 || y >= (int)INPUT_HEIGHT) return STATE_FLOOR;
	return gol->arena[GOL_INDEX(x,y)];
}

static inline void gol_set(gol_t* restrict gol, int x,int y, enum gol_state st)
{
	if (x<0 || x >= (int)INPUT_WIDTH || y<0 || y >= (int)INPUT_HEIGHT) return;

	gol->arena[GOL_INDEX(x,y)] = st;
}

static inline int count_states(const enum gol_state* state, enum gol_state s, size_t n)
{
	register int j=0;
	for(register size_t i=0;i<n;i++)
		j+= (state[i] == s);
	return j;
}

static inline int gol_count(const gol_t* gol, enum gol_state s)
{
	return count_states(gol->arena, s, INPUT_WIDTH*INPUT_HEIGHT);
}

static inline void neighbours(const gol_t* gol, int x, int y, enum gol_state out[restrict 8])
{
	out[0] = gol_index(gol, x-1, y-1);
	out[1] = gol_index(gol, x, y-1);
	out[2] = gol_index(gol, x+1, y-1);

	out[3] = gol_index(gol, x-1, y);
	//out[4] = gol_index(gol, x, y);
	out[4] = gol_index(gol, x+1, y);

	out[5] = gol_index(gol, x-1, y+1);
	out[6] = gol_index(gol, x, y+1);
	out[7] = gol_index(gol, x+1, y+1);
}

static void simulate_ip(const gol_t* source, gol_t* restrict dest)
{
	for(size_t h=0;h<INPUT_HEIGHT;h++)
		for(size_t w=0;w<INPUT_WIDTH;w++) {
			enum gol_state next = gol_index(source, w, h);
			enum gol_state n[8];
			neighbours(source, w, h, n);
			switch(next) {
			case STATE_EMPTY:
				if(count_states(n, STATE_OCCU, 8)==0) next = STATE_OCCU;
				break;
			case STATE_OCCU:
				if(count_states(n, STATE_OCCU, 8)>=4) next = STATE_EMPTY;
			default: break;
			}
			gol_set(dest, w, h, next);
		}
}

static inline int gol_eq(const gol_t* g1, const gol_t* g2)
{
	return memcmp(g1->arena, g2->arena, sizeof(g1->arena))==0;
}

gol_t simulate_once(const gol_t* source)
{
	gol_t out = *source;
	simulate_ip(source, &out);
	
	return out;
}

#ifdef DEBUG
static void print_arena(const gol_t* gol)
{
	for(size_t h=0;h<INPUT_HEIGHT;h++) {
		for(size_t w=0;w<INPUT_WIDTH;w++) fputc((char)gol_index(gol, w, h), stderr);
		fputc('\n', stderr);
	}
}
#endif

int main()
{
	gol_t* arena = box(gol_t);
	*arena = generate_arena();

#ifdef DEBUG
	printf("First: \n");
	print_arena(arena);
#endif
	gol_t tmp;
	while(1)
	{
		tmp = simulate_once(arena);
#ifdef DEBUG
		printf("Next: \n");
		print_arena(&tmp);
#endif
		if(gol_eq(&tmp, arena))
		{
			printf("%d\n", gol_count(&tmp, STATE_OCCU));
			break;
		}
		*arena = tmp;
	}
	free(arena);
	return 0;
}
