// ERROR typechecker

// There was a bug where we forgot to check the type
// of the argument expression so this makes sure that will never happen again
int f(int* x) {
    return *x;
}

int main(int arg) {
    int x = 5;

    // ??
    f(&x / 5);
    return 0;
}
