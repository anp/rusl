#include <stdio.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <signal.h>
#include <setjmp.h>

#define TEST2(c, l, ...) ((c) ? 1 : \
(err++,printf(__FILE__":"#l": "#c" failed: " __VA_ARGS__),putchar('\n'),0))
#define TEST1(c, l, ...) TEST2(c, l, __VA_ARGS__)
#define TEST(c, ...) TEST1(c, __LINE__, __VA_ARGS__)

#define TESTE(c) TEST(c, "errno = %s", strerror(errno))

int test_setjmp(void)
{
	volatile int err = 0;
	volatile int x = 0, r;
	jmp_buf jb;
	sigjmp_buf sjb;
	volatile sigset_t oldset;
	sigset_t set;

	if (!setjmp(jb)) {
		x = 1;
		longjmp(jb, 1);
	}
	TEST(x==1, "setjmp/longjmp seems to have been bypassed");

	x = 0;
	r = setjmp(jb);
	if (!x) {
		x = 1;
		longjmp(jb, 0);
	}
	TEST(r==1, "longjmp(jb, 0) caused setjmp to return %d", r);

	sigemptyset(&set);
	sigaddset(&set, SIGUSR1);
	sigprocmask(SIG_UNBLOCK, &set, &set);
	oldset = set;

	/* Improve the chances of catching failure of sigsetjmp to
	 * properly save the signal mask in the sigjmb_buf. */
	memset(&sjb, -1, sizeof sjb);

	if (!sigsetjmp(sjb, 1)) {
		sigemptyset(&set);
		sigaddset(&set, SIGUSR1);
		sigprocmask(SIG_BLOCK, &set, 0);
		siglongjmp(sjb, 1);
	}
	set = oldset;
	sigprocmask(SIG_SETMASK, &set, &set);
	TEST(sigismember(&set, SIGUSR1)==0, "siglongjmp failed to restore mask");

	return err;
}
