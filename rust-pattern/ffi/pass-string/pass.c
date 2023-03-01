#include <stdio.h>

void set_err(char *message) {
    printf("err: %s\n", message);
}

// #define TEST 1
#ifdef  TEST 
int main() {
    set_err("some error");
}
#endif