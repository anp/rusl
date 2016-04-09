#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <semaphore.h>
#include <time.h>
#include <fcntl.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

int test_sem(void)
{
	int err = 0;
	int r;
	void *res;
	char buf[100];
	struct timespec ts;
	sem_t *sem, *sem2;
	int val;

	clock_gettime(CLOCK_REALTIME, &ts);
	snprintf(buf, sizeof buf, "/testsuite-%d-%d", (int)getpid(), (int)ts.tv_nsec);

	TEST(r, !(sem=sem_open(buf, O_CREAT|O_EXCL, 0700, 1)), 0, "could not open sem");

	TEST(r, sem_getvalue(sem, &val), 0, "failed to get sem value");
	TEST(r, val, 1, "wrong initial semaphore value");

	TEST(r, !(sem2=sem_open(buf, 0)), 0, "could not reopen sem");
	TEST(r, sem!=sem2, 0, "reopened sem has different address");

	TEST(r, sem_wait(sem), 0, "failed on sem wait");
	TEST(r, sem_getvalue(sem2, &val), 0, "failed to get sem value");
	TEST(r, val, 0, "wrong semaphore value on second handle");

	TEST(r, sem_post(sem), 0, "failed on sem post");
	TEST(r, sem_getvalue(sem2, &val), 0, "failed to get sem value");
	TEST(r, val, 1, "wrong semaphore value on second handle");

	TEST(r, sem_close(sem), 0, "failed to close sem");
	TEST(r, sem_close(sem), 0, "failed to close sem second time");
	TEST(r, sem_unlink(buf), 0, "failed to unlink sem");

	return err;
}
