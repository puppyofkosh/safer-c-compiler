// ERROR typechecker

fn int function(int arg) {
    return 0;
}

fn int main(int arg) {
    let pointer(int) p = &call(function, 5);

    return 0;
}
