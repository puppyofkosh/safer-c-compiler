// ERROR typechecker
int main(int arg) {
    int x = 5;
    
    // No!
    int* y = &x / 10;

    return 0;
}
