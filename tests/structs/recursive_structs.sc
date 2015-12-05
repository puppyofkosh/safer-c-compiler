// ERROR typechecker

// You definitely are not allowed to do this.
struct A {
       B x;
}

struct B {
       A x;
}

int main(int arg) {
    A a;

    return 0;
}
