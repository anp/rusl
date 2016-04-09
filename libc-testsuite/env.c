#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <limits.h>
#include <unistd.h>
#include <stdlib.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x, strerror(errno)), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

extern char **environ;
int clearenv(void);

int test_env(void)
{
	int i;
	int err=0;
	char a[100], b[100], *s;
	FILE *f;

	TEST(i, (clearenv(), !environ || !*environ), 1, "failed");
	TEST(i, putenv("TEST=1"), 0, "failed");
	TEST(s, environ[1], 0, "%p != 0");
	TEST_S((s=getenv("TEST")), "1", "failed");
	TEST(i, unsetenv("TEST"), 0, "failed");
	TEST(i, !*environ, 1, "failed");
	TEST(i, setenv("TEST", "2", 0), 0, "failed");
	TEST_S((s=getenv("TEST")), "2", "failed");
	TEST(i, setenv("TEST", "3", 0), 0, "failed");
	TEST_S((s=getenv("TEST")), "2", "failed");
	TEST(i, setenv("TEST", "3", 1), 0, "failed");
	TEST_S((s=getenv("TEST")), "3", "failed");

	return err;
}
