#include <stdio.h>

#define RUN_TEST(a) { \
extern int test_ ##a (void); \
int e = test_ ##a (); \
if (e) printf("%s test failed, %d error(s)\n", #a, e); \
else   printf("%s test passed\n", #a); \
err += e; \
}

int main()
{
	int err=0;

	RUN_TEST(fdopen);
	RUN_TEST(fcntl);
	RUN_TEST(fnmatch);
	RUN_TEST(fscanf);
	RUN_TEST(popen);
	RUN_TEST(socket);
	RUN_TEST(spawn);
	RUN_TEST(qsort);
	RUN_TEST(time);
	RUN_TEST(sscanf);
	RUN_TEST(snprintf);
	RUN_TEST(swprintf);
	RUN_TEST(stat);
	RUN_TEST(string);
	RUN_TEST(strtod);
	RUN_TEST(strtol);
	RUN_TEST(ungetc);
	RUN_TEST(wcstol);
	RUN_TEST(fwscanf);
	RUN_TEST(basename);
	RUN_TEST(dirname);
	RUN_TEST(memstream);
	RUN_TEST(mbc);
	RUN_TEST(setjmp);
	RUN_TEST(sem);
	RUN_TEST(pthread);
	/* env is last because it will break subsequent tests */
	RUN_TEST(env);

	printf("\ntotal errors: %d\n", err);
	return !!err;
}
