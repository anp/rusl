#include <sys/stat.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <time.h>
#include <stdint.h>
#include <unistd.h>

#define TEST2(c, l, ...) ((c) ? 1 : \
(err++,printf(__FILE__":"#l": "#c" failed: " __VA_ARGS__),putchar('\n'),0))
#define TEST1(c, l, ...) TEST2(c, l, __VA_ARGS__)
#define TEST(c, ...) TEST1(c, __LINE__, __VA_ARGS__)

int test_stat(void)
{
	int err = 0;
	struct stat st;
	FILE *f;
	time_t t;

	if (TEST(stat(".",&st)==0, "errno = %s", strerror(errno))) {
		TEST(S_ISDIR(st.st_mode), "");
		TEST(st.st_nlink>0, "%ju", (uintmax_t)st.st_nlink);
		t = time(0);
		TEST(st.st_ctime<=t, "%jd > %jd", (intmax_t)st.st_ctime, (intmax_t)t);
		TEST(st.st_mtime<=t, "%jd > %jd", (intmax_t)st.st_mtime, (intmax_t)t);
		TEST(st.st_atime<=t, "%jd > %jd", (intmax_t)st.st_atime, (intmax_t)t);
	}

	if (TEST(stat("/dev/null",&st)==0, "errno = %s", strerror(errno))) {
		TEST(S_ISCHR(st.st_mode), "");
	}

	if ((f = tmpfile())) {
		fputs("hello", f);
		fflush(f);
		if (TEST(fstat(fileno(f),&st)==0, "errnp = %s", strerror(errno))) {
			TEST(st.st_uid==geteuid(), "%d vs %d", (int)st.st_uid, (int)geteuid());
			TEST(st.st_gid==getegid(), "%d vs %d", (int)st.st_uid, (int)geteuid());
			TEST(st.st_size==5, "%jd vs 5", (intmax_t)st.st_size);
		}
		fclose(f);
	}

	return err;
}
