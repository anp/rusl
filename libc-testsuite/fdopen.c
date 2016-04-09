#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_E(f) ( (errno = 0), (f) || \
(printf(__FILE__ ":%d: %s failed (errno = %d)\n", __LINE__, #f, errno), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

int test_fdopen(void)
{
	char tmp[] = "/tmp/testsuite-XXXXXX";
	char foo[6];
	int fd;
	int err = 0;
	FILE *f;

	TEST_E((fd = mkstemp(tmp)) > 2);
	TEST_E(write(fd, "hello", 6)==6);
	TEST_E(f = fdopen(fd, "rb"));
	if (f) {
		TEST_E(ftello(f)==6);
		TEST_E(fseeko(f, 0, SEEK_SET)==0);
		TEST_E(fgets(foo, sizeof foo, f));
		TEST_S(foo, "hello", "fgets read back wrong message");
		fclose(f);
	}

	if (fd > 2) TEST_E(unlink(tmp) != -1);

	return err;
}
