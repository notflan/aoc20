#ifndef _PANIC_H
#define _PANIC_H

#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

#include "attrs.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wall"
noinline cold static noreturn panic(const char* msg, ...)
{
	va_list va;
	va_start(va, msg);
	fputs("Fatal error: ", stderr);
	vfprintf(stderr, msg, va);
	va_end(va);
	fputs("\n", stderr);
	abort();
}
#pragma GCC diagnostic pop

#endif /* _PANIC_H */
