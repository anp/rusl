#include <stdio.h>
#include <string.h>
#include <wchar.h>
#include <stdlib.h>
#include <locale.h>
#include <langinfo.h>

/* r = place to store result
 * f = function call to test (or any expression)
 * x = expected result
 * m = message to print on failure (with formats for r & x)
**/

#define TEST(r, f, x, m) ( \
memset(&st, 0, sizeof st), \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

int test_mbc(void)
{
	char b[32];
	char *s;
	const char *cs;
	int i;
	int err=0;
	mbstate_t st, st2;
	wchar_t wc, wcs[32];
	
	setlocale(LC_CTYPE, "en_US.UTF-8") ||
	setlocale(LC_CTYPE, "en_GB.UTF-8") ||
	setlocale(LC_CTYPE, "en.UTF-8") ||
	setlocale(LC_CTYPE, "POSIX.UTF-8") ||
	setlocale(LC_CTYPE, "C.UTF-8") ||
	setlocale(LC_CTYPE, "UTF-8") ||
	setlocale(LC_CTYPE, "");

	TEST(i, mbsrtowcs(wcs, (cs="abcdef",&cs), 3, &st), 3, "wrong semantics for wcs buf len, %d != %d");
	TEST(i, mbsrtowcs(wcs, (cs="abcdef",&cs), 8, &st), 6, "wrong semantics for wcs buf len, %d != %d");
	TEST(i, mbsrtowcs(NULL, (cs="abcdef",&cs), 2, &st), 6, "wrong semantics for NULL wcs, %d != %d");

	if (strcmp(nl_langinfo(CODESET), "UTF-8")) {
		printf(__FILE__ ": cannot set UTF-8 locale for test"
			" (codeset=%s)\n", nl_langinfo(CODESET));
		return 0;
	}
	
	TEST(i, mbrtowc(&wc, "\x80", 1, &st), -1, "failed to catch error %d != %d");
	TEST(i, mbrtowc(&wc, "\xc0", 1, &st), -1, "failed to catch illegal initial, %d != %d");

	TEST(i, mbrtowc(&wc, "\xc0\x80", 2, &st), -1, "aliasing nul %d != %d");
	TEST(i, mbrtowc(&wc, "\xc0\xaf", 2, &st), -1, "aliasing slash %d != %d");
	TEST(i, mbrtowc(&wc, "\xe0\x80\xaf", 3, &st), -1, "aliasing slash %d != %d");
	TEST(i, mbrtowc(&wc, "\xf0\x80\x80\xaf", 4, &st), -1, "aliasing slash %d != %d");
	TEST(i, mbrtowc(&wc, "\xf8\x80\x80\x80\xaf", 5, &st), -1, "aliasing slash %d != %d");
	TEST(i, mbrtowc(&wc, "\xfc\x80\x80\x80\x80\xaf", 6, &st), -1, "aliasing slash %d != %d");
	TEST(i, mbrtowc(&wc, "\xe0\x82\x80", 3, &st), -1, "aliasing U+0080 %d != %d");
	TEST(i, mbrtowc(&wc, "\xe0\x9f\xbf", 3, &st), -1, "aliasing U+07FF %d != %d");
	TEST(i, mbrtowc(&wc, "\xf0\x80\xa0\x80", 4, &st), -1, "aliasing U+0800 %d != %d");
	TEST(i, mbrtowc(&wc, "\xf0\x8f\xbf\xbd", 4, &st), -1, "aliasing U+FFFD %d != %d");

	TEST(i, mbrtowc(&wc, "\xed\xa0\x80", 3, &st), -1, "failed to catch surrogate, %d != %d");
	TEST(i, mbrtowc(&wc, "\xef\xbf\xbe", 3, &st), 3, "failed to accept U+FFFE, %d != %d");
	TEST(i, mbrtowc(&wc, "\xef\xbf\xbf", 3, &st), 3, "failed to accept U+FFFF, %d != %d");
	TEST(i, mbrtowc(&wc, "\xf4\x8f\xbf\xbe", 4, &st), 4, "failed to accept U+10FFFE, %d != %d");
	TEST(i, mbrtowc(&wc, "\xf4\x8f\xbf\xbf", 4, &st), 4, "failed to accept U+10FFFF, %d != %d");

	TEST(i, mbrtowc(&wc, "\xc2\x80", 2, &st), 2, "wrong length %d != %d");
	TEST(i, (mbrtowc(&wc, "\xc2\x80", 2, &st),wc), 0x80, "wrong char %04x != %04x");
	TEST(i, mbrtowc(&wc, "\xe0\xa0\x80", 3, &st), 3, "wrong length %d != %d");
	TEST(i, (mbrtowc(&wc, "\xe0\xa0\x80", 3, &st),wc), 0x800, "wrong char %04x != %04x");
	TEST(i, mbrtowc(&wc, "\xf0\x90\x80\x80", 4, &st), 4, "wrong length %d != %d");
	TEST(i, (mbrtowc(&wc, "\xf0\x90\x80\x80", 4, &st),wc), 0x10000, "wrong char %04x != %04x");

	memset(&st2, 0, sizeof st2);
	TEST(i, mbrtowc(&wc, "\xc2", 1, &st2), -2, "failed to accept initial byte, %d != %d");
	TEST(i, mbrtowc(&wc, "\x80", 1, &st2), 1, "failed to resume, %d != %d");
	TEST(i, wc, 0x80, "wrong char %04x != %04x");

	memset(&st2, 0, sizeof st2);
	TEST(i, mbrtowc(&wc, "\xc2", 1, &st2), -2, "failed to accept initial byte, %d != %d");
	TEST(i, mbsrtowcs(wcs, (cs="\xa0""abc",&cs), 32, &st2), 4, "failed to resume, %d != %d");
	TEST(i, wcs[0], 0xa0, "wrong char %04x != %04x");
	TEST(i, wcs[1], 'a', "wrong char %04x != %04x");
	TEST(i, !cs, 1, "wrong final position %d != %d");

	return err;
}

