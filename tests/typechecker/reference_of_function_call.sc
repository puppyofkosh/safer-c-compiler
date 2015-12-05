// ERROR typechecker

int function(int arg) {
    return 0;
}

int main(int arg) {
    let pointer(int) p = &call(function, 5);

    return 0;
}
