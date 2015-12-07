// 579
int main(int arg) {
    int sz = 2;
    int* x = allocate(4 * sz);

    *x = 123;
    *(x + 1) = 456;

    int result = *x + *(x + 1);

    printf("%d\n", result);

    free(x);

    return 0;
}
