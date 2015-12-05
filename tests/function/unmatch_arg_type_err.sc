// ERROR typechecker

int func(int a, int b) {
    return 0;
}

int main(int arg) {
    int a = 10;
    pointer(int) b = &a;

    func(a, b);

    return 0;
}
