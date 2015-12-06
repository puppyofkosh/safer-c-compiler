// ERROR typechecker
int main(int arg) {
    int x = 5;


    if 1 == 1 {
        int y = 10;

        // Not allowed
        int x = 6;
    }

    return 0;
}
