#define _GNU_SOURCE
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <stdint.h>
#include <errno.h>
#include <sys/mman.h>
#include "libc.h"
#include "atomic.h"
#include "pthread_impl.h"

#if defined(__GNUC__) && defined(__PIC__)
#define inline inline __attribute__((always_inline))
#endif

void *__mmap(void *, size_t, int, int, int, off_t);
int __munmap(void *, size_t);
void *__mremap(void *, size_t, size_t, int, ...);
int __madvise(void *, size_t, int);

struct chunk {
	size_t psize, csize;
	struct chunk *next, *prev;
};

struct bin {
	volatile int lock[2];
	struct chunk *head;
	struct chunk *tail;
};

struct {
	volatile uint64_t binmap;
	struct bin bins[64];
	volatile int free_lock[2];
} mal;


#define SIZE_ALIGN (4*sizeof(size_t))
#define SIZE_MASK (-SIZE_ALIGN)
#define OVERHEAD (2*sizeof(size_t))
#define MMAP_THRESHOLD (0x1c00*SIZE_ALIGN)
#define DONTCARE 16
#define RECLAIM 163840

#define CHUNK_SIZE(c) ((c)->csize & -2)
#define CHUNK_PSIZE(c) ((c)->psize & -2)
#define PREV_CHUNK(c) ((struct chunk *)((char *)(c) - CHUNK_PSIZE(c)))
#define NEXT_CHUNK(c) ((struct chunk *)((char *)(c) + CHUNK_SIZE(c)))
#define MEM_TO_CHUNK(p) (struct chunk *)((char *)(p) - OVERHEAD)
#define CHUNK_TO_MEM(c) (void *)((char *)(c) + OVERHEAD)
#define BIN_TO_CHUNK(i) (MEM_TO_CHUNK(&mal.bins[i].head))

#define C_INUSE  ((size_t)1)

#define IS_MMAPPED(c) !((c)->csize & (C_INUSE))


/* Synchronization tools */

static inline void lock(volatile int *lk)
{
	if (libc.threads_minus_1)
		while(a_swap(lk, 1)) __wait(lk, lk+1, 1, 1);
}

static inline void unlock(volatile int *lk)
{
	if (lk[0]) {
		a_store(lk, 0);
		if (lk[1]) __wake(lk, 1, 1);
	}
}

void lock_bin(int i)
{
	lock(mal.bins[i].lock);
	if (!mal.bins[i].head)
		mal.bins[i].head = mal.bins[i].tail = BIN_TO_CHUNK(i);
}

void unlock_bin(int i)
{
	unlock(mal.bins[i].lock);
}

int first_set(uint64_t x)
{
	return a_ctz_64(x);
}

int bin_index(size_t x)
{
	x = x / SIZE_ALIGN - 1;
	if (x <= 32) return x;
	if (x > 0x1c00) return 63;
	return ((union { float v; uint32_t r; }){(int)x}.r>>21) - 496;
}

int bin_index_up(size_t x)
{
	x = x / SIZE_ALIGN - 1;
	if (x <= 32) return x;
	return ((union { float v; uint32_t r; }){(int)x}.r+0x1fffff>>21) - 496;
}

void *__expand_heap(size_t *);
struct chunk *expand_heap(size_t n);
int adjust_size(size_t *n);
void unbin(struct chunk *c, int i);
int alloc_fwd(struct chunk *c);
int alloc_rev(struct chunk *c);
int pretrim(struct chunk *self, size_t n, int i, int j);
void trim(struct chunk *self, size_t n);
void *malloc(size_t n);
void *__malloc0(size_t n);
void *realloc(void *p, size_t n);

void free(void *p)
{
	struct chunk *self = MEM_TO_CHUNK(p);
	struct chunk *next;
	size_t final_size, new_size, size;
	int reclaim=0;
	int i;

	if (!p) return;

	if (IS_MMAPPED(self)) {
		size_t extra = self->psize;
		char *base = (char *)self - extra;
		size_t len = CHUNK_SIZE(self) + extra;
		/* Crash on double free */
		if (extra & 1) a_crash();
		__munmap(base, len);
		return;
	}

	final_size = new_size = CHUNK_SIZE(self);
	next = NEXT_CHUNK(self);

	/* Crash on corrupted footer (likely from buffer overflow) */
	if (next->psize != self->csize) a_crash();

	for (;;) {
		if (self->psize & next->csize & C_INUSE) {
			self->csize = final_size | C_INUSE;
			next->psize = final_size | C_INUSE;
			i = bin_index(final_size);
			lock_bin(i);
			lock(mal.free_lock);
			if (self->psize & next->csize & C_INUSE)
				break;
			unlock(mal.free_lock);
			unlock_bin(i);
		}

		if (alloc_rev(self)) {
			self = PREV_CHUNK(self);
			size = CHUNK_SIZE(self);
			final_size += size;
			if (new_size+size > RECLAIM && (new_size+size^size) > size)
				reclaim = 1;
		}

		if (alloc_fwd(next)) {
			size = CHUNK_SIZE(next);
			final_size += size;
			if (new_size+size > RECLAIM && (new_size+size^size) > size)
				reclaim = 1;
			next = NEXT_CHUNK(next);
		}
	}

	if (!(mal.binmap & 1ULL<<i))
		a_or_64(&mal.binmap, 1ULL<<i);

	self->csize = final_size;
	next->psize = final_size;
	unlock(mal.free_lock);

	self->next = BIN_TO_CHUNK(i);
	self->prev = mal.bins[i].tail;
	self->next->prev = self;
	self->prev->next = self;

	/* Replace middle of large chunks with fresh zero pages */
	if (reclaim) {
		uintptr_t a = (uintptr_t)self + SIZE_ALIGN+PAGE_SIZE-1 & -PAGE_SIZE;
		uintptr_t b = (uintptr_t)next - SIZE_ALIGN & -PAGE_SIZE;
		__madvise((void *)a, b-a, MADV_DONTNEED);
	}

	unlock_bin(i);
}
