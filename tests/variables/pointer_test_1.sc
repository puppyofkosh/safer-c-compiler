// 2;
fn main(arg) {
   let int x = 123;
   let int y = &x;

   let int g = 100 + 24 - *y;
   g = g + 124 - *y;

   print g;
   return 0;
}
