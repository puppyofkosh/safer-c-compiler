
int fib(int n) {
    if (n == 0) { return 0; }
    if (n == 1) { return 1; }
    return fib(n-1) + fib(n-2);
}

int main(int arg) {
    printf("Which fibonacci number do you want to look up?[0-40]:");
    int n;
    scanf("%d", &n);
    printf("fib(%d) = %d\n", n, fib(n));
    return 0;
}
