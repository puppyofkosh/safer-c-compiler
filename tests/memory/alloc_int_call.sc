// 11
fn int main(int arg) {
    let pointer(int) p = alloc_int(1);

    *p = 11;

    print *p;
    free_int(p);

    return 0;
}
