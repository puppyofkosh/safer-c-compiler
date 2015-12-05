// ERROR typechecker

int func(int a, int b) {
    return 0;
}

int main(int arg) {
    let int a = 10;
    let pointer(int) b = &a;

    func(a, b);

    return 0;
}
