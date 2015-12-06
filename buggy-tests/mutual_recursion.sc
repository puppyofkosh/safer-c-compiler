// 10

int g(int x) {
    if x == 10 {
        return 10;
    }

    return f(x + 1);
}

int f(int x) {
    if x == 10 { 
        return 10;
    }

    return g(x + 1);
}


int main(int arg) {
    print g(0);

    return 0;
}
