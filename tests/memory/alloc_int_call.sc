// 11
int main(int arg) {
    pointer(int) p = alloc_int(1);

    *p = 11;

    print *p;
    free_int(p);

    return 0;
}
