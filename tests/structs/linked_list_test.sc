// 6

struct List {
       int v;
       List* next;
}

int sum_list(List* p) {
    int s = 0;
    int *null = 0;
    while p != null {
        s = s + (*p).v;
        p = (*p).next;
    }

    return s;
}

int main(int arg) {
    List a;
    a.v = 1;
    a.next = 0;

    List b;
    b.v = 2;
    b.next = &a;

    List c;
    c.v = 3;
    c.next = &b;

    print sum_list(&c);

    return 0;
}
