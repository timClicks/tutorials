#include <stdio.h>

int main() {
    int x = 0;
    
    if ((x = 10)) {
        printf("x has been modified, it's now %d\n", x);
    } else {
        printf("x is still zero\n");
    }

    return 0;
}

