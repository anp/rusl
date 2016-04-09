#include <pthread.h>
#include <semaphore.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

static pthread_key_t k1, k2;

static void dtor(void *p)
{
	*(int *)p = 1;
}

static void *start1(void *arg)
{
	return arg;
}

static void *start2(void *arg)
{
	int *p = arg;
	if (pthread_setspecific(k1, p) || pthread_setspecific(k2, p+1))
		return arg;
	return 0;
}

static void *start3(void *arg)
{
	pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS, 0);
	sem_post(arg);
	for (;;);
	return 0;
}

static void cleanup4(void *arg)
{
	*(int *)arg = 1;
}

static void *start4(void *arg)
{
	pthread_cleanup_push(cleanup4, arg);
	sleep(3);
	pthread_cleanup_pop(0);
	return 0;
}

static void cleanup4a2(void *arg)
{
	*(int *)arg += 2;
}

static void cleanup4a3(void *arg)
{
	*(int *)arg += 3;
}

static void cleanup4a4(void *arg)
{
	*(int *)arg += 4;
}

static void *start4a(void *arg)
{
	int *foo = arg;
	pthread_cleanup_push(cleanup4, foo);
	pthread_cleanup_push(cleanup4a2, foo+1);
	pthread_cleanup_push(cleanup4a3, foo+2);
	pthread_cleanup_push(cleanup4a4, foo+3);
	sleep(3);
	pthread_cleanup_pop(0);
	pthread_cleanup_pop(0);
	pthread_cleanup_pop(0);
	pthread_cleanup_pop(0);
	return 0;
}

static void *start5(void *arg)
{
	pthread_mutex_lock(arg);
	return 0;
}

static void *start6(void *arg)
{
	void **args = arg;
	pthread_mutex_lock(args[1]);
	pthread_barrier_wait(args[0]);
	nanosleep(&(struct timespec){ .tv_nsec = 10000000 }, 0);
	return 0;
}

static void *start7(void *arg)
{
	void **args = arg;
	pthread_mutex_lock(args[1]);
	pthread_cond_signal(args[0]);
	pthread_mutex_unlock(args[1]);
	return 0;
}

static void *start8(void *arg)
{
	void **args = arg;
	pthread_mutex_t *m = args[1];
	pthread_cond_t *c = args[0];
	int *x = args[2];

	pthread_mutex_lock(m);
	while (*x) pthread_cond_wait(c, m);
	pthread_mutex_unlock(m);

	return 0;
}


int test_pthread(void)
{
	pthread_t td, td1, td2, td3;
	int err = 0;
	int r;
	void *res;
	int foo[4], bar[2];
	pthread_barrier_t barrier2;
	pthread_mutexattr_t mtx_a;
	pthread_mutex_t mtx, *sh_mtx;
	pthread_cond_t cond;
	sem_t sem1;
	int fd;

	TEST(r, pthread_barrier_init(&barrier2, 0, 2), 0, "creating barrier");
	TEST(r, sem_init(&sem1, 0, 0), 0, "creating semaphore");

	/* Test basic thread creation and joining */
	TEST(r, pthread_create(&td, 0, start1, &res), 0, "failed to create thread");
	res = 0;
	TEST(r, pthread_join(td, &res), 0, "failed to join");
	TEST(r, (res==&res), 1, "wrong result from join");

	/* Test POSIX thread-specific data */
	TEST(r, pthread_key_create(&k1, dtor), 0, "failed to create key");
	TEST(r, pthread_key_create(&k2, dtor), 0, "failed to create key");
	foo[0] = foo[1] = 0;
	TEST(r, pthread_setspecific(k1, bar), 0, "failed to set tsd");
	TEST(r, pthread_setspecific(k2, bar+1), 0, "failed to set tsd");
	TEST(r, pthread_create(&td, 0, start2, foo), 0, "failed to create thread");
	TEST(r, pthread_join(td, &res), 0, "failed to join");
	TEST(res, res, 0, "pthread_setspecific failed in thread");
	TEST(r, foo[0], 1, "dtor failed to run");
	TEST(r, foo[1], 1, "dtor failed to run");
	TEST(res, pthread_getspecific(k1), bar, "tsd corrupted");
	TEST(res, pthread_getspecific(k2), bar+1, "tsd corrupted");
	TEST(r, pthread_setspecific(k1, 0), 0, "failed to clear tsd");
	TEST(r, pthread_setspecific(k2, 0), 0, "failed to clear tsd");
	TEST(r, pthread_key_delete(k1), 0, "failed to destroy key");
	TEST(r, pthread_key_delete(k2), 0, "failed to destroy key");

	/* Asynchronous cancellation */
	TEST(r, pthread_create(&td, 0, start3, &sem1), 0, "failed to create thread");
	while (sem_wait(&sem1));
	TEST(r, pthread_cancel(td), 0, "canceling");
	TEST(r, pthread_join(td, &res), 0, "joining canceled thread");
	TEST(res, res, PTHREAD_CANCELED, "canceled thread exit status");

	/* Cancellation cleanup handlers */
	foo[0] = 0;
	TEST(r, pthread_create(&td, 0, start4, foo), 0, "failed to create thread");
	TEST(r, pthread_cancel(td), 0, "cancelling");
	TEST(r, pthread_join(td, &res), 0, "joining canceled thread");
	TEST(res, res, PTHREAD_CANCELED, "canceled thread exit status");
	TEST(r, foo[0], 1, "cleanup handler failed to run");

	/* Nested cleanup handlers */
	memset(foo, 0, sizeof foo);
	TEST(r, pthread_create(&td, 0, start4a, foo), 0, "failed to create thread");
	TEST(r, pthread_cancel(td), 0, "cancelling");
	TEST(r, pthread_join(td, &res), 0, "joining canceled thread");
	TEST(res, res, PTHREAD_CANCELED, "canceled thread exit status");
	TEST(r, foo[0], 1, "cleanup handler failed to run");
	TEST(r, foo[1], 2, "cleanup handler failed to run");
	TEST(r, foo[2], 3, "cleanup handler failed to run");
	TEST(r, foo[3], 4, "cleanup handler failed to run");

	/* Robust mutexes */
	TEST(r, pthread_mutexattr_init(&mtx_a), 0, "initializing mutex attr");
	TEST(r, pthread_mutexattr_setrobust(&mtx_a, PTHREAD_MUTEX_ROBUST), 0, "setting robust attribute");
	TEST(r, pthread_mutex_init(&mtx, &mtx_a), 0, "initializing robust mutex");
	TEST(r, pthread_mutex_lock(&mtx), 0, "locking robust mutex");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "unlocking robust mutex");
	TEST(r, pthread_create(&td, 0, start5, &mtx), 0, "failed to create thread");
	TEST(r, pthread_join(td, &res), 0, "joining thread");
	TEST(r, pthread_mutex_lock(&mtx), EOWNERDEAD, "locking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "unlocking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_lock(&mtx), ENOTRECOVERABLE, "re-locking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "destroying unrecoverable mutex %d!=%d");

	TEST(r, pthread_mutex_init(&mtx, &mtx_a), 0, "initializing robust mutex");
	TEST(r, pthread_create(&td, 0, start5, &mtx), 0, "failed to create thread");
	TEST(r, pthread_join(td, &res), 0, "joining thread");
	TEST(r, pthread_mutex_lock(&mtx), EOWNERDEAD, "locking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_consistent(&mtx), 0, "%d!=%d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "unlocking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "re-locking orphaned robust mutex %d!=%d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "destroying mutex %d!=%d");

	TEST(r, pthread_mutex_init(&mtx, &mtx_a), 0, "%d != %d");
	TEST(r, pthread_create(&td, 0, start6, (void *[]){ &barrier2, &mtx }), 0, "%d != %d");
	pthread_barrier_wait(&barrier2);
	TEST(r, pthread_mutex_lock(&mtx), EOWNERDEAD, "%d != %d");
	TEST(r, pthread_join(td, &res), 0, "%d != %d");
	TEST(r, pthread_mutex_consistent(&mtx), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "%d != %d");

	//TEST(r, (fd=open("/dev/zero", O_RDWR))>=0, 1, "opening zero page file");
	//TEST(r, 

	/* Condition variables */
	TEST(r, pthread_mutex_init(&mtx, 0), 0, "%d != %d");
	TEST(r, pthread_cond_init(&cond, 0), 0, "%d != %d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	TEST(r, pthread_create(&td, 0, start7, (void *[]){ &cond, &mtx }), 0, "%d != %d");
	TEST(r, pthread_cond_wait(&cond, &mtx), 0, "%d != %d");
	TEST(r, pthread_join(td, &res), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_destroy(&cond), 0, "%d != %d");

	/* Condition variables with multiple waiters */
	TEST(r, pthread_mutex_init(&mtx, 0), 0, "%d != %d");
	TEST(r, pthread_cond_init(&cond, 0), 0, "%d != %d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	foo[0] = 1;
	TEST(r, pthread_create(&td1, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_create(&td2, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_create(&td3, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	nanosleep(&(struct timespec){.tv_nsec=1000000}, 0);
	foo[0] = 0;
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_signal(&cond), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_signal(&cond), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_signal(&cond), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_join(td1, 0), 0, "%d != %d");
	TEST(r, pthread_join(td2, 0), 0, "%d != %d");
	TEST(r, pthread_join(td3, 0), 0, "%d != %d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_destroy(&cond), 0, "%d != %d");

	/* Condition variables with broadcast signals */
	TEST(r, pthread_mutex_init(&mtx, 0), 0, "%d != %d");
	TEST(r, pthread_cond_init(&cond, 0), 0, "%d != %d");
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	foo[0] = 1;
	TEST(r, pthread_create(&td1, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_create(&td2, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_create(&td3, 0, start8, (void *[]){ &cond, &mtx, foo }), 0, "%d != %d");
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	nanosleep(&(struct timespec){.tv_nsec=1000000}, 0);
	TEST(r, pthread_mutex_lock(&mtx), 0, "%d != %d");
	foo[0] = 0;
	TEST(r, pthread_mutex_unlock(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_broadcast(&cond), 0, "%d != %d");
	TEST(r, pthread_join(td1, 0), 0, "%d != %d");
	TEST(r, pthread_join(td2, 0), 0, "%d != %d");
	TEST(r, pthread_join(td3, 0), 0, "%d != %d");
	TEST(r, pthread_mutex_destroy(&mtx), 0, "%d != %d");
	TEST(r, pthread_cond_destroy(&cond), 0, "%d != %d");

	return err;
}
