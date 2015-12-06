// ERROR typechecker

int some_function(int a, int b, int* c) {
    return 3;
}

int main(int arg) {
    int x = 5;

    int y = 10;
    
    // shouldn't pass &y as 2nd arg
    some_function(x, &y, &x);

    return 0;
}
