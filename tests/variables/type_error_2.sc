// ERROR typechecker
fn int id_to_int(char a) {
   return a;
}

fn int main(int arg) {
    let char x = 5;
    x = call(id_to_int, x);

    return x;
}
