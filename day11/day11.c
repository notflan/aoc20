#include <stdlib.h>

#include <panic.h>
#include <attrs.h>

const char* const input[] = {
#include "input.h"
};

// input_sz //
#include "input-sz.h"
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

int main()
{
	gol_t* arena = box(gol_t);
	*arena = generate_arena();

	printf("%lu\n", sizeof(arena));

	free(arena);
	return 0;
}
