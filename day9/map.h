#ifndef _MAP_H
#define _MAP_H

#ifndef KELEMENT
#include <stdint.h>
#	define KELEMENT uint64_t
#endif
#ifndef VELEMENT
#include <stddef.h>
#	define VELEMENT size_t
#endif

typedef KELEMENT map_key_t;
typedef VELEMENT map_value_t;

#define PAGE_SIZE UINT16_MAX

typedef struct _map {
	struct page* page0;
} map_t;

map_t map_new();
void map_insert(map_t * restrict into, map_key_t k, map_value_t v);
bool map_get(const map_t* from, map_key_t k, map_value_t* restrict v);
void map_free(map_t map);
void map_clear(map_t* restrict map);

#endif /* _MAP_H */
