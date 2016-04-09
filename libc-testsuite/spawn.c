#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <spawn.h>
#include <sys/wait.h>

#define TEST(r, f, x, m) ( \
((r) = (f)) == (x) || \
(printf(__FILE__ ":%d: %s failed (" m ")\n", __LINE__, #f, r, x), err++, 0) )

#define TEST_E(f) ( (errno = 0), (f) || \
(printf(__FILE__ ":%d: %s failed (errno = %d)\n", __LINE__, #f, errno), err++, 0) )

#define TEST_S(s, x, m) ( \
!strcmp((s),(x)) || \
(printf(__FILE__ ":%d: [%s] != [%s] (%s)\n", __LINE__, s, x, m), err++, 0) )

int test_spawn(void)
{
	int r;
	char foo[10];
	int p[2];
	pid_t pid;
	int status;
	int err = 0;
	posix_spawnattr_t attr;
	posix_spawn_file_actions_t fa;

	TEST_E(!pipe(p));
	TEST(r, posix_spawn_file_actions_init(&fa), 0, "%d != %d");
	TEST(r, posix_spawn_file_actions_addclose(&fa, p[0]), 0, "%d != %d");
	TEST(r, posix_spawn_file_actions_adddup2(&fa, p[1], 1), 0, "%d != %d");
	TEST(r, posix_spawn_file_actions_addclose(&fa, p[1]), 0, "%d != %d");
	TEST(r, posix_spawnp(&pid, "echo", &fa, 0, (char *[]){"echo","hello",0}, 0), 0, "%d != %d");
	close(p[1]);
	TEST(r, waitpid(pid, &status, 0), pid, "%d != %d");
	TEST(r, read(p[0], foo, sizeof foo), 6, "%d != %d");
	close(p[0]);
	TEST(r, posix_spawn_file_actions_destroy(&fa), 0, "%d != %d");

	return err;
}
