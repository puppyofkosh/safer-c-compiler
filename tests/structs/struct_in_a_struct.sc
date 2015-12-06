// 300

// Test having a struct inside another struct.
// Make sure writing to the fields doesn't cause any sort of accidental
// overwrite

struct A {
    int x;
    int y;
}

struct B {
    A x;
    A y;
}

int compute_sum(B* p) {
    int result = (*p).x.x + (*p).x.y + (*p).y.x + (*p).y.y;
    return result;
}

int main(int arg) {
    B b;

    b.x.x = 10;
    b.x.y = 10;
    b.y.x = 40;
    b.y.y = 40;

    // 100
    int s = compute_sum(&b);

    // Now write to b.x.y, and make sure nothing else gets changed
    int* p = &b.x.y;
    *p = *p + 100;

    // 300
    s = s + compute_sum(&b);

    print s;

    return 0;
}
