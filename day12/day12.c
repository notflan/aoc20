#include <stdint.h>

typedef uint64_t u64;

const char* const input[] = {
#include "input.h"
};
#define input_sz (sizeof(input)/sizeof(char*))

typedef struct {
	u64 x, y;
} point_t;

typedef struct {
	enum direction {
		DIR_NORTH,
		DIR_SOUTH,
		DIR_EAST,
		DIR_WEST,

		DIR_LEFT,
		DIR_RIGHT,

		DIR_FORWARD,
	} dir;
	u64 num;
} command_t;

typedef struct ship {
	point_t pos;
	enum direction facing;
} svec_t;


int main()
{
	return 0;
}

