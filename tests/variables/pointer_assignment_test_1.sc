// 2
fn main(arg) {
   let int x = 10;
   let int y = &x;

   *y = 2;

   print x;
   return 0;
}
