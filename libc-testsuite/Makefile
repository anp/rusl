
SRCS = $(wildcard *.c)
OBJS = $(SRCS:.c=.o)

CFLAGS = -std=c99 -Os -fno-builtin -D_XOPEN_SOURCE=700
#LDFLAGS = -static
LIBS = -lrt -lpthread -lrt -lm

-include config.mak

all: testsuite

testsuite: $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $(OBJS) $(LIBS)

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $<

clean:
	rm -f $(OBJS) testsuite

relink:
	rm -f testsuite
	make all

test: testsuite
	./testsuite

retest: relink test
