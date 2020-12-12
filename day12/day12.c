#include <stdint.h>
#include <stdlib.h>

#include <panic.h>
#include <attrs.h>

typedef int64_t i64;
typedef uint64_t u64;

const char* const input[] = {
#include "input.h"
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

static void handle_com(struct ship* restrict ship, command_t com)
{
	switch(com.dir)
	{
	case DIR_EAST:  ship->pos.x += com.num; break;
	case DIR_WEST:  ship->pos.x -= com.num; break;
	case DIR_NORTH: ship->pos.y -= com.num; break;
	case DIR_SOUTH: ship->pos.y += com.num; break;
	
	case DIR_FORWARD: modsign(ship, com.num); break;

	case DIR_LEFT: //TODO:
	case DIR_RIGHT: //TODO:

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

