// 5

int main(int arg) {
    owned_pointer(int) o = alloc_owned_int(4);

    *o = 5;

    if 1 == 1 {
        owned_pointer(int) o2 = alloc_owned_int(4);

        *o2 = 6;
        print *o2;
    }

    print *o;
    return 0;
}
