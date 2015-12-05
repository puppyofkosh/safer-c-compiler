// 5;
fn int f2(int arg) {
   print arg;
   return 0;
}

fn int f1(int arg) {
   f2(arg+1);
   return 0;
}

fn int main(int arg) {
   f1(4);
   return 0;
}
