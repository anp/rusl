#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <wchar.h>

#define TEST(r, f, x, m) ( \
msg = #f, ((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST2(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, msg, r, x), err++, 0) )

int test_wcstol(void)
{
	int i;
	long l;
	unsigned long ul;
	long long ll;
	unsigned long long ull;
	char *msg="";
	int err=0;
	wchar_t *s, *c;

	TEST(l, wcstol(L"2147483647", 0, 0), 2147483647L, "max 32bit signed %ld != %ld");
	TEST(ul, wcstoul(L"4294967295", 0, 0), 4294967295UL, "max 32bit unsigned %lu != %lu");

	if (sizeof(long) == 4) {
		errno = 0;
		TEST(l, wcstol(s=L"2147483648", &c, 0), 2147483647L, "uncaught overflow %ld != %ld");
		TEST2(i, c-s, 10, "wrong final position %d != %d");
		TEST2(i, errno, ERANGE, "missing errno %d != %d");
		errno = 0;
		TEST(l, wcstol(s=L"-2147483649", &c, 0), -2147483647L-1, "uncaught overflow %ld != %ld");
		TEST2(i, c-s, 11, "wrong final position %d != %d");
		TEST2(i, errno, ERANGE, "missing errno %d != %d");
		errno = 0;
		TEST(ul, wcstoul(s=L"4294967296", &c, 0), 4294967295UL, "uncaught overflow %lu != %lu");
		TEST2(i, c-s, 10, "wrong final position %d != %d");
		TEST2(i, errno, ERANGE, "missing errno %d != %d");
		errno = 0;
		TEST(ul, wcstoul(s=L"-1", &c, 0), -1UL, "rejected negative %lu != %lu");
		TEST2(i, c-s, 2, "wrong final position %d != %d");
		TEST2(i, errno, 0, "spurious errno %d != %d");
		errno = 0;
		TEST(ul, wcstoul(s=L"-2", &c, 0), -2UL, "rejected negative %lu != %lu");
		TEST2(i, c-s, 2, "wrong final position %d != %d");
		TEST2(i, errno, 0, "spurious errno %d != %d");
		errno = 0;
		TEST(ul, wcstoul(s=L"-2147483648", &c, 0), -2147483648UL, "rejected negative %lu != %lu");
		TEST2(i, c-s, 11, "wrong final position %d != %d");
		TEST2(i, errno, 0, "spurious errno %d != %d");
		errno = 0;
		TEST(ul, wcstoul(s=L"-2147483649", &c, 0), -2147483649UL, "rejected negative %lu != %lu");
		TEST2(i, c-s, 11, "wrong final position %d != %d");
		TEST2(i, errno, 0, "spurious errno %d != %d");
	} else {
		TEST(i, 0, 1, "64bit tests not implemented");
	}

	TEST(l, wcstol(L"z", 0, 36), 35, "%ld != %ld");
	TEST(l, wcstol(L"00010010001101000101011001111000", 0, 2), 0x12345678, "%ld != %ld");

	TEST(l, wcstol(s=L"0xz", &c, 16), 0, "%ld != %ld");
	TEST2(i, c-s, 1, "wrong final position %ld != %ld");

	TEST(l, wcstol(s=L"0x1234", &c, 16), 0x1234, "%ld != %ld");
	TEST2(i, c-s, 6, "wrong final position %ld != %ld");

	errno = 0;
	c = NULL;
	TEST(l, wcstol(s=L"123", &c, 37), 0, "%ld != %ld");
	TEST2(i, c-s, 0, "wrong final position %d != %d");
	TEST2(i, errno, EINVAL, "%d != %d");

	return err;
}
