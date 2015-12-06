// ERROR typechecker

struct A {
    int x;
    int y;
}

// not allowed to return structs yet
A function(int x) {
    A a;
    a.x = x;
    return a;
}

int main(int arg) {
    print function(5);
    return 0;
}
