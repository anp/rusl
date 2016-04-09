#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <math.h>

/* r = place to store result
 * f = function call to test (or any expression)
 * x = expected result
 * m = message to print on failure (with formats for r & x)
**/

#define TEST(r, f, x, m) ( \
msg = #f, ((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x, r-x), err++, 0) )

#define TEST2(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, msg, r, x), err++, 0) )

int test_strtod(void)
{
	int i;
	double d, d2;
	char buf[1000];
	char *msg="";
	int err=0;
	char *s, *c;

	for (i=0; i<100; i++) {
		d = sin(i);
		snprintf(buf, sizeof buf, "%.300f", d);
		TEST(d2, strtod(buf, 0), d, "round trip fail %a != %a (%a)");
	}

	TEST(d, strtod("0x1p4", 0), 16.0, "hex float %a != %a");
	TEST(d, strtod("0x1.1p4", 0), 17.0, "hex float %a != %a");

	return err;
}
