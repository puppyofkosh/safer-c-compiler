// 123
int main(int arg) {
    int sz = 2;
    int* x = allocate(4 * sz);

    *x = 123;
    *(x + 1) = 456;

    int* y = x + 1;

    printf("%d\n", *(y - 1));

    free(x);

    return 0;
}
