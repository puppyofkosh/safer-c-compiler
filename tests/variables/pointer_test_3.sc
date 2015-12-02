// 2
fn int get_val(pointer(int) arg) {
   return *arg;
}

fn int main(int arg) {
   let int x = 2;
   print call(get_val, &x);
   return 0;
}
