// 5
fn setFive(arg) {
   *arg = 5;
   return 0;
}

fn main(arg) {
   let int x = 11;
   call(setFive, &x);

   print x;
   return 0;
}
