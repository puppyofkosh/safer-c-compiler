// 2
fn int getVal(pointer(int) arg) {
   return *arg;
}

fn int main(int arg) {
   let int x = 2;
   print call(getVal, &x);
   return 0;
}
