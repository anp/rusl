#include <stdio.h>
#include <string.h>
#include <libgen.h>
#include <stdlib.h>

#define TEST(p, b) ( \
tmp = strdup((p)), s = dirname(tmp), \
!strcmp((b),s) || \
(printf(__FILE__ ":%d: dirname(\"%s\") returned \"%s\"; expected \"%s\"\n", \
__LINE__, (p), s, (b)), err++, 0), free(tmp), 0 )

int test_dirname(void)
{
	char *tmp, *s;
	int err=0;

	if (strcmp(dirname(NULL), ".")) {
		printf(__FILE__ ":%d: dirname(NULL) returned \"%s\"; "
			"expected \".\"\n", __LINE__, dirname(NULL));
		err++;
	}
	TEST("", ".");
	TEST("/usr/lib", "/usr");
	TEST("/usr/", "/");
	TEST("usr", ".");
	TEST("/", "/");
	TEST("///", "/");
	TEST(".", ".");
	TEST("..", ".");

	return err;
}
