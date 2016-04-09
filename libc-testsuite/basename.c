#include <stdio.h>
#include <string.h>
#include <libgen.h>
#include <stdlib.h>

#define TEST(p, b) ( \
tmp = strdup((p)), s = basename(tmp), \
!strcmp((b),s) || \
(printf(__FILE__ ":%d: basename(\"%s\") returned \"%s\"; expected \"%s\"\n", \
__LINE__, (p), s, (b)), err++, 0), free(tmp), 0 )

int test_basename(void)
{
	char *tmp, *s;
	int err=0;

	if (strcmp(basename(NULL), ".")) {
		printf(__FILE__ ":%d: basename(NULL) returned \"%s\"; "
			"expected \".\"\n", __LINE__, basename(NULL));
		err++;
	}
	TEST("", ".");
	TEST("/usr/lib", "lib");
	TEST("/usr/", "usr");
	TEST("/", "/");
	TEST("///", "/");
	TEST("//usr//lib//", "lib");

	return err;
}
