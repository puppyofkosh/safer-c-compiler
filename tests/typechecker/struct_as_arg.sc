// ERROR typechecker

struct A {
    int x;
    int y;
}

// not allowed to pass structs as args yet
int function(A a) {
    return a.x;
}

int main(int arg) {
    A a;
    print function(a);
    return 0;
}
