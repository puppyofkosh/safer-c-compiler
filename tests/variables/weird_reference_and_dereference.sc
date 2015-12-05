// 3
int main(int arg) {
    int x = 3;
    pointer(int) pa = &x;

    // Weird, but it should be allowed
    pointer(int) pb = &*pa;
    pointer(int) pc = &*pb;

    print *pc;
}
