// 55

// This test is mostly to make sure that this compiles

struct A {
    pointer(B) b;
}

struct B {
    A a;
    int x;       
}

fn int main(int arg) {
    let B b;
    b.a.b = &b;
    b.x = 55;
    
    print b.x;

    return 0;
}
