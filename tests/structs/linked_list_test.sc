// 6

struct List {
       int v;
       pointer(List) next;
}

fn int sum_list(pointer(List) p) {
    let int s = 0;
    while p != 0 {
        s = s + (*p).v;
        p = (*p).next;
    }

    return s;
}

fn int main(int arg) {
    let List a;
    a.v = 1;
    a.next = 0;

    let List b;
    b.v = 2;
    b.next = &a;

    let List c;
    c.v = 3;
    c.next = &b;

    print call(sum_list, &c);
    
    return 0;
}
