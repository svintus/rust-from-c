#include <stdio.h>

extern int add(int a, int b);

int main(int argc, char **argv) {
  int value = add(2, 3);
  printf("hello world! %d\n", value);
  return 0;
}
