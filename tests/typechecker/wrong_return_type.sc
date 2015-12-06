// ERROR typechecker

int some_function(int a, int b) {
    // no
    return &a;
}

int main(int arg) {
    int x = 5;

    int y = 10;
    
    some_function(x, y);

    return 0;
}
