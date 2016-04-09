#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <sys/wait.h>

#define TEST2(c, l, ...) ((c) ? 1 : \
(err++,printf(__FILE__":"#l": "#c" failed: " __VA_ARGS__),putchar('\n'),0))
#define TEST1(c, l, ...) TEST2(c, l, __VA_ARGS__)
#define TEST(c, ...) TEST1(c, __LINE__, __VA_ARGS__)

#define TESTE(c) TEST(c, "errno = %s", strerror(errno))

int test_fcntl(void)
{
	int err = 0;
	struct flock fl = {0};
	FILE *f;
	int fd;
	pid_t pid;
	int status;

	if (!TESTE(f=tmpfile())) return err;
	fd = fileno(f);

	fl.l_type = F_WRLCK;
	fl.l_whence = SEEK_SET;
	fl.l_start = 0;
	fl.l_len = 0;
	TESTE(fcntl(fd, F_SETLK, &fl)==0);

	pid = fork();
	if (!pid) {
		fl.l_type = F_RDLCK;
		_exit(fcntl(fd, F_SETLK, &fl)==0 ||
			(errno!=EAGAIN && errno!=EACCES));
	}
	while (waitpid(pid, &status, 0)<0 && errno==EINTR);
	TEST(status==0, "lock failed to work");

	pid = fork();
	if (!pid) {
		fl.l_type = F_WRLCK;
		_exit(fcntl(fd, F_GETLK, &fl) || fl.l_pid != getppid());
	}
	while (waitpid(pid, &status, 0)<0 && errno==EINTR);
	TEST(status==0, "child failed to detect lock held by parent");

	fclose(f);

	return err;
}
