#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <limits.h>
#include <unistd.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x, strerror(errno)), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

int test_ungetc(void)
{
	int i;
	int err=0;
	char a[100], b[100], *s;
	FILE *f;

	TEST(i, !(f = tmpfile()), 0, "failed to create temp file %d!=%d (%s)");

	if (!f) return err;

	errno=0;
	TEST(i, fprintf(f, "hello, world\n"), 13, "%d != %d (%m)");
	TEST(i, fseek(f, 0, SEEK_SET), 0, "%d != %d (%m)");

	TEST(i, feof(f), 0, "%d != %d");
	TEST(i, fgetc(f), 'h', "'%c' != '%c'");
	TEST(i, ftell(f), 1, "%d != %d");
	TEST(i, ungetc('x', f), 'x', "%d != %d");
	TEST(i, ftell(f), 0, "%d != %d");
	TEST(i, fscanf(f, "%[h]", a), 0, "got %d fields, expected %d");
	TEST(i, ftell(f), 0, "%d != %d");
	TEST(i, fgetc(f), 'x', "'%c' != '%c'");
	TEST(i, ftell(f), 1, "%d != %d");

	TEST(i, fseek(f, 0, SEEK_SET), 0, "%d != %d");
	TEST(i, ungetc('x', f), 'x', "%d != %d");
	TEST(i, fread(a, 1, sizeof a, f), 14, "read %d, expected %d");
	a[14] = 0;
	TEST_S(a, "xhello, world\n", "mismatch reading ungot character");

	TEST(i, fseek(f, 0, SEEK_SET), 0, "%d != %d");
	TEST(i, fscanf(f, "%[x]", a), 0, "got %d fields, expected %d");
	TEST(i, ungetc('x', f), 'x', "unget failed after fscanf: %d != %d");
	TEST(i, fgetc(f), 'x', "'%c' != '%c'");
	TEST(i, fgetc(f), 'h', "'%c' != '%c'");

	fclose(f);

	return err;
}
