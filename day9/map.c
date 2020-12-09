#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "map.h"
/*
static inline void* align_ptr(void* ptr, size_t al)
{
	unsigned char* buffer = ptr;
	return buffer + al - ((intptr_t)buffer) % al;
}
*/
#ifdef _EXPR_EXT			
#define box(t) ({ t* restrict p = aligned_alloc(_Alignof(t), sizeof(t)); \
			*p = (t){0}; \
		p; })
#elif !defined(IGNORE_ALIGMENT)
inline extern void* _zero_ptr(void* ptr, size_t n)
{
	memset(ptr, 0, n);
	return ptr;
}
#define box(t) _zero_ptr(aligned_alloc(_Alignof(t), sizeof(t)), sizeof(t))
#else
#define box(t) calloc(sizeof(t), 1)
#endif

struct page {
	struct entry {
		bool set;
		map_key_t key;
		map_value_t value;
	} p[PAGE_SIZE];

	struct page* next;
};

map_t map_new()
{
	register map_t rval;

	rval.page0 = box(struct page);
	return rval;
}

static inline size_t page_index(map_key_t k)
{
	return (size_t)(k % PAGE_SIZE);
}

void map_insert(map_t * restrict into, map_key_t k, map_value_t v)
{
	struct page** p = &into->page0;
	size_t i = page_index(k);

	while(*p) {
		register struct page* page = *p;
		if (!page->p[i].set || page->p[i].key == k)
		{
			page->p[i] = (struct entry){.set = true, .key = k, .value = v};
			return;
		}
		p = &page->next;
	}
	// p == &<last page>.next == &NULL
	(*p = box(struct page))->p[i] = (struct entry){.set = true, .key = k, .value = v};
}

bool map_get(const map_t* from, map_key_t k, map_value_t* restrict v)
{
	const struct page* p = from->page0;
	size_t i = page_index(k);

	while(p) {
		if (p->p[i].set && p->p[i].key ==k) {
			*v =  p->p[i].value;
			return true;
		}
		p = p->next;
	}
	return false;
}

void map_free(map_t map)
{
	while(map.page0)
	{
		struct page* next = map.page0->next;
		free(map.page0);
		map.page0 = next;
	}
}

static void _map_walk(map_t* restrict map, void (*cb)(struct page* page))
{
	struct page* page=  map->page0;
	while(page)
	{
		cb(page);
		page = page->next;
	}
}

static void _map_page_clear(struct page* page)
{
	memset(page, 0, sizeof(struct page));
}

void map_clear(map_t* restrict map)
{
	_map_walk(map, &_map_page_clear);
}
