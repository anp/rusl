#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <limits.h>
#include <math.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

static const struct {
	const char *fmt;
	int i;
	const char *expect;
} int_tests[] = {
	/* width, precision, alignment */
	{ "%04d", 12, "0012" },
	{ "%.3d", 12, "012" },
	{ "%3d", 12, " 12" },
	{ "%-3d", 12, "12 " },
	{ "%+3d", 12, "+12" },
	{ "%+-5d", 12, "+12  " },
	{ "%+- 5d", 12, "+12  " },
	{ "%- 5d", 12, " 12  " },
	{ "% d", 12, " 12" },
	{ "%0-5d", 12, "12   " },
	{ "%-05d", 12, "12   " },

	/* ...explicit precision of 0 shall be no characters. */
	{ "%.0d", 0, "" },
	{ "%.0o", 0, "" },
	{ "%#.0d", 0, "" },
	{ "%#.0o", 0, "" },
	{ "%#.0x", 0, "" },

	/* ...but it still has to honor width and flags. */
	{ "%2.0u", 0, "  " },
	{ "%02.0u", 0, "  " },
	{ "%2.0d", 0, "  " },
	{ "%02.0d", 0, "  " },
	{ "% .0d", 0, " " },
	{ "%+.0d", 0, "+" },

	/* hex: test alt form and case */
	{ "%x", 63, "3f" },
	{ "%#x", 63, "0x3f" },
	{ "%X", 63, "3F" },

	/* octal: test alt form */
	{ "%o", 15, "17" },
	{ "%#o", 15, "017" },

	{ NULL, 0.0, NULL }
};

static const struct {
	const char *fmt;
	double f;
	const char *expect;
} fp_tests[] = {
	/* basic form, handling of exponent/precision for 0 */
	{ "%e", 0.0, "0.000000e+00" },
	{ "%f", 0.0, "0.000000" },
	{ "%g", 0.0, "0" },
	{ "%#g", 0.0, "0.00000" },

	/* rounding */
	{ "%f", 1.1, "1.100000" },
	{ "%f", 1.2, "1.200000" },
	{ "%f", 1.3, "1.300000" },
	{ "%f", 1.4, "1.400000" },
	{ "%f", 1.5, "1.500000" },
	{ "%.4f", 1.06125, "1.0612" },
	{ "%.2f", 1.375, "1.38" },
	{ "%.1f", 1.375, "1.4" },
	{ "%.15f", 1.1, "1.100000000000000" },
	{ "%.16f", 1.1, "1.1000000000000001" },
	{ "%.17f", 1.1, "1.10000000000000009" },
	{ "%.2e", 1500001.0, "1.50e+06" },
	{ "%.2e", 1505000.0, "1.50e+06" },
	{ "%.2e", 1505000.00000095367431640625, "1.51e+06" },
	{ "%.2e", 1505001.0, "1.51e+06" },
	{ "%.2e", 1506000.0, "1.51e+06" },
	
	/* correctness in DBL_DIG places */
	{ "%.15g", 1.23456789012345, "1.23456789012345" },

	/* correct choice of notation for %g */
	{ "%g", 0.0001, "0.0001" },
	{ "%g", 0.00001, "1e-05" },
	{ "%g", 123456, "123456" },
	{ "%g", 1234567, "1.23457e+06" },
	{ "%.7g", 1234567, "1234567" },
	{ "%.7g", 12345678, "1.234568e+07" },
	{ "%.8g", 0.1, "0.1" },
	{ "%.9g", 0.1, "0.1" },
	{ "%.10g", 0.1, "0.1" },
	{ "%.11g", 0.1, "0.1" },

	/* pi in double precision, printed to a few extra places */
	{ "%.15f", M_PI, "3.141592653589793" },
	{ "%.18f", M_PI, "3.141592653589793116" },

	/* exact conversion of large integers */
	{ "%.0f", 340282366920938463463374607431768211456.0,
	         "340282366920938463463374607431768211456" },

	{ NULL, 0.0, NULL }
};

int test_snprintf(void)
{
	int i, j, k;
	int err=0;
	char b[2000], *s;

	TEST(i, snprintf(0, 0, "%d", 123456), 6, "length returned %d != %d");
	TEST(i, snprintf(0, 0, "%.4s", "hello"), 4, "length returned %d != %d");
	TEST(i, snprintf(b, 0, "%.0s", "goodbye"), 0, "length returned %d != %d");

	strcpy(b, "xxxxxxxx");
	TEST(i, snprintf(b, 4, "%d", 123456), 6, "length returned %d != %d");
	TEST_S(b, "123", "incorrect output");
	TEST(i, b[5], 'x', "buffer overrun");

	/* Perform ascii arithmetic to test printing tiny doubles */
	TEST(i, snprintf(b, sizeof b, "%.1022f", 0x1p-1021), 1024, "%d != %d");
	b[1] = '0';
	for (i=0; i<1021; i++) {
		for (k=0, j=1023; j>0; j--) {
			if (b[j]<'5') b[j]+=b[j]-'0'+k, k=0;
			else b[j]+=b[j]-'0'-10+k, k=1;
		}
	}
	TEST(i, b[1], '1', "'%c' != '%c'");
	for (j=2; b[j]=='0'; j++);
	TEST(i, j, 1024, "%d != %d");


#ifndef DISABLE_SLOW_TESTS
	errno = 0;
	TEST(i, snprintf(NULL, 0, "%.*u", 2147483647, 0), 2147483647, "cannot print max length %d");
	TEST(i, snprintf(NULL, 0, "%.*u ", 2147483647, 0), -1, "integer overflow %d");
	TEST(i, errno, EOVERFLOW, "after overflow: %d != %d");
#endif
	for (j=0; int_tests[j].fmt; j++) {
		TEST(i, snprintf(b, sizeof b, int_tests[j].fmt, int_tests[j].i), strlen(b), "%d != %d");
		TEST_S(b, int_tests[j].expect, "bad integer conversion");
	}

	for (j=0; fp_tests[j].fmt; j++) {
		TEST(i, snprintf(b, sizeof b, fp_tests[j].fmt, fp_tests[j].f), strlen(b), "%d != %d");
		TEST_S(b, fp_tests[j].expect, "bad floating point conversion");
	}

	TEST(i, snprintf(0, 0, "%.4a", 1.0), 11, "%d != %d");

	return err;
}
