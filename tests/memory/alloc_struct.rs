// 5 10 123 456
struct A {
    int x;
    int y;
}

A* alloc_a(int num) {
    A* a = allocate(8 * num);
    return a;
}


int main(int arg) {
    A* a = alloc_a(2);

    (*a).x = 5;
    (*a).y = 10;

    (*(a + 1)).x = 123;
    (*(a + 1)).y = 456;
    
    A* b = a + 1;

    printf("%d %d %d %d\n", (*a).x, (*a).y, (*b).x, (*b).y);

    free(a);

    return 0;
}
