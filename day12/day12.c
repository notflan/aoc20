#include <stdint.h>
#include <stdlib.h>

#include <panic.h>
#include <attrs.h>

typedef int64_t i64;
typedef uint64_t u64;

const char* const input[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};
#define input_sz (sizeof(input)/sizeof(char*))

typedef struct {
	i64 x, y;
} point_t;

typedef struct {
	enum direction {
		DIR_INVALID =0,

		DIR_NORTH = 'N',
		DIR_SOUTH = 'S',
		DIR_EAST  = 'E',
		DIR_WEST  = 'W',

		DIR_LEFT  = 'L',
		DIR_RIGHT = 'R',

		DIR_FORWARD='F',
	} dir;
	i64 num;
} command_t;

typedef struct ship {
	point_t pos;
	enum direction facing;
} svec_t;

pure inline static command_t parse_single(const char* from)
{
	register command_t o;
	o.dir = (enum direction)from[0];
	o.num = (i64)atoll(from+1);
	return o;
}

inline static void modsign(struct ship* restrict ship, i64 by)
{
	i64* restrict mod;
	i64 sign = 1;
	switch(ship->facing)
	{
		case DIR_NORTH: sign = -1; fall;
		case DIR_SOUTH: mod = &ship->pos.y;
				break;

		case DIR_WEST: sign = -1; fall;
		case DIR_EAST: mod = &ship->pos.x;
			       break;
		default: panic("Invalid facing direction '%c' (%d)", (char) ship->facing, (int) ship->facing);
	}
	*mod += sign * by;
}

noglobal static inline enum direction rotr(enum direction s)
{
#define DIRECT(n) return n; case n
	switch(s)
	{
	case DIR_NORTH:
		DIRECT(DIR_EAST):
		DIRECT(DIR_SOUTH):
		DIRECT(DIR_WEST):
		return DIR_NORTH;
	default: panic("Cannot rotate direction '%c', (%d)", (char)s, (int) s);
	}
}
noglobal static inline enum direction rotl(enum direction s)
{
	switch(s)
	{
	case DIR_NORTH:
		DIRECT(DIR_WEST):
		DIRECT(DIR_SOUTH):
		DIRECT(DIR_EAST):
		return DIR_NORTH;
	default: panic("Cannot rotate direction '%c', (%d)", (char)s, (int) s);
	}
}
#undef DIRECT


static void handle_com(struct ship* restrict ship, command_t com)
{
	switch(com.dir)
	{
	case DIR_EAST:  ship->pos.x += com.num; break;
	case DIR_WEST:  ship->pos.x -= com.num; break;
	case DIR_NORTH: ship->pos.y -= com.num; break;
	case DIR_SOUTH: ship->pos.y += com.num; break;
	/* XXX: LEFT and RIGHT's num is TURN BY DEGREES, not turn, then move!?!?!??? TODO redo
	case DIR_LEFT: ship->facing = rotl(ship->facing); if(0)
	case DIR_RIGHT: ship->facing = rotr(ship->facing);
	*/
	case DIR_FORWARD: modsign(ship, com.num); break;
	default: panic("Unknown command direction '%c' (%d)", (char)com.dir, (int)com.dir);
	}
}

inline extern u64 absll(i64 n)
{
	return (u64)(n < 0 ? -n : n);
}

int main()
{
	struct ship ship = { .facing = DIR_EAST, .pos = {0} };
	//command_t comms[input_sz];
	for(size_t i=0;i<input_sz;i++)
		handle_com(&ship, parse_single(input[i]));
	
	printf("%lu\n", absll(ship.pos.x) + absll(ship.pos.y));
	return 0;
}

