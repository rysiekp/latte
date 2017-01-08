#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char *concat(char *s1, char *s2) {
    size_t s1_size = strlen(s1);
    size_t s2_size = strlen(s2);
    char *res = malloc(s1_size + s2_size + 1);
    if (res == NULL) {
        return NULL;
    }
    strcpy(res, s1);
    strcpy(res + s1_size, s2);
    res[s1_size + s2_size + 1] = 0;
    return res;
}

void printString(char *s) {
    printf("%s\n", s);
}

void printInt(int x) {
    printf("%d\n", x);
}

int readInt() {
    int x;
    scanf("%d", &x);
    getchar();
    return x;
}

char *readString() {
    char *res = NULL;
    size_t size = 0;
    if (getline(&res, &size, stdin) == -1) {
        return NULL;
    }
    size_t len = strlen(res);
    if (res[0] != '\n') {
        res[len - 1] = 0;
    }
    return res;
}

void error() {
    printf("runtime error\n");
    exit(-1);
}