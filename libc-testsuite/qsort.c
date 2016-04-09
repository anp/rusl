#include <stdlib.h>
#include <stdio.h>
#include <string.h>

static int scmp(const void *a, const void *b)
{
	return strcmp(*(char **)a, *(char **)b);
}

static int icmp(const void *a, const void *b)
{
	return *(int*)a - *(int*)b;
}

#define FAIL(m) (printf(__FILE__ ":%d: %s failed\n", __LINE__, m), err++, 0)

int test_qsort(void)
{
	int i;
	int err=0;
	/* 26 items -- even */
	char *s[] = {
		"Bob", "Alice", "John", "Ceres",
		"Helga", "Drepper", "Emeralda", "Zoran",
		"Momo", "Frank", "Pema", "Xavier",
		"Yeva", "Gedun", "Irina", "Nono",
		"Wiener", "Vincent", "Tsering", "Karnica",
		"Lulu", "Quincy", "Osama", "Riley",
		"Ursula", "Sam"
	};
	/* 23 items -- odd, prime */
	int n[] = {
		879045, 394, 99405644, 33434, 232323, 4334, 5454,
		343, 45545, 454, 324, 22, 34344, 233, 45345, 343,
		848405, 3434, 3434344, 3535, 93994, 2230404, 4334
	};

	qsort(s, sizeof(s)/sizeof(char *), sizeof(char *), scmp);
	for (i=0; i<sizeof(s)/sizeof(char *)-1; i++) {
		if (strcmp(s[i], s[i+1]) > 0) {
			FAIL("string sort");
			for (i=0; i<sizeof(s)/sizeof(char *); i++)
				printf("\t%s\n", s[i]);
			break;
		}
	}

	qsort(n, sizeof(n)/sizeof(int), sizeof(int), icmp);
	for (i=0; i<sizeof(n)/sizeof(int)-1; i++) {
		if (n[i] > n[i+1]) {
			FAIL("integer sort");
			for (i=0; i<sizeof(n)/sizeof(int); i++)
				printf("\t%d\n", n[i]);
			break;
		}
	}

	return err;
}
