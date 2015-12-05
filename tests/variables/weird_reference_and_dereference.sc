// 3
fn int main(int arg) {
    let int x = 3;
    let pointer(int) pa = &x;

    // Weird, but it should be allowed
    let pointer(int) pb = &*pa;
    let pointer(int) pc = &*pb;

    print *pc;
}
