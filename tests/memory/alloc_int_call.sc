// 11
fn int main(int arg) {
    let pointer(int) p = call(alloc_int, 1);

    *p = 11;

    print *p;
    call(free_int, p);

    return 0;
}
