// 21
struct MyStruct {
    int x;
    int y;
}

int compute_sum(pointer(MyStruct) p) {
    return (*p).x + (*p).y;
}

int main(int arg) {
    // Pad the stack with extra stuff to make sure all
    // the offset computations work right
    int space;
    int space2;

    MyStruct s;

    s.x = 1;
    s.y = 20;

    // meaningless statements which will cause us to push/pop to the stack
    if s.x == s.y + 10 {
        int space3;
    }

    print compute_sum(&s);

    return 0;
}
