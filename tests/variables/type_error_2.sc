// ERROR typechecker
int id_to_int(char a) {
   return a;
}

int main(int arg) {
    char x = 5;
    x = call(id_to_int, x);

    return x;
}

