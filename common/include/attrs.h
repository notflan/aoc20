#ifndef _ATTRS_H
#define _ATTRS_H

#define pure __attribute__((pure))
#define noglobal __attribute__((const))
#define noinline __attribute__((noinline))
#define cold __attribute__((cold))

#ifndef DEBUG
#define _force_inline __attribute__((gnu_inline)) inline extern
#else
#define _force_inline __attribute__((gnu_inline)) inline static
#endif

#ifndef __cplusplus // sepples has [[noreturn]]
#define noreturn __attribute__((noreturn)) void
#endif

#endif /* _ATTRS_H */
