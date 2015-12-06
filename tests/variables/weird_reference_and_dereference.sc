// 3
int main(int arg) {
    int x = 3;
    int* pa = &x;

    // Weird, but it should be allowed
    int* pb = &*pa;
    int* pc = &*pb;

    print *pc;
}
