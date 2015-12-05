// 113
struct A {
    char x;
    int y;
}


int main(int arg) {
    A a;

    a.x = 1;
    a.y = 2;

    int result = 0;

    pointer(char) p = &a.x;

    // Test reading from the pointer (1)
    result = result + *p;

    // Test writing to it (11)
    *p = 10;
    result = result + a.x;

    // Make sure we didn't accidentally change the value of y (13)
    result = result + a.y;

    // Now try changing a.x and reading from the pointer (113)
    a.x = 100;
    result = result + *p;

    print result;
    return 0;
}
