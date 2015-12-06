// ERROR typechecker

int function(int arg) {
    return 0;
}

int main(int arg) {
    int* p = &call(function, 5);

    return 0;
}
