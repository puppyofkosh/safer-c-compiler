// 5
fn int setFive(pointer(int) arg) {
   *arg = 5;
   return 0;
}

fn int main(int arg) {
   let int x = 11;
   call(setFive, &x);

   print x;
   return 0;
}
