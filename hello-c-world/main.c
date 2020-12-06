#include <stdio.h>

extern char *get_string();
int main() {
  char *string = get_string();
  printf("%s", string);
}
