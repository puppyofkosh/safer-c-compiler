// 55

// This test is mostly to make sure that this compiles

struct A {
    pointer(B) b;
}

struct B {
    A a;
    int x;       
}

int main(int arg) {
    B b;
    b.a.b = &b;
    b.x = 55;
    
    print b.x;

    return 0;
}
