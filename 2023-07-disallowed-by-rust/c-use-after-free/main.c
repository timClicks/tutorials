#include <stdlib.h>

int main() {
    int *p = (int *) malloc(sizeof(int));
    *p = 10;
    free(p);

    *p = 20;
    
    return 0;
}
