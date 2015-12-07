// 45
int main(int arg) {
    int* x = allocate(4 * 10);
    
    int i = 0;
    while i < 10 {
        *(x + i) = i;
        i = i + 1;
    }

    i = 0;
    int sum = 0;
    while i < 10 {
        sum = sum + *(x + i);
        i = i + 1;
    }

    print sum;

    free(x);
    return 0;
}
