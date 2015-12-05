// ERROR typechecker

fn int func(int a, int b) {
    return 0;
}

fn int main(int arg) {
    let int a = 10;
    let pointer(int) b = &a;

    func(a, b);

    return 0;
}
