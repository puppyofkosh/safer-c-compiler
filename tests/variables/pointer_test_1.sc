// 2;
fn int main(int arg) {
   let int x = 123;
   let pointer(int) y = &x;

   let int g = 100 + 24 - *y;
   g = g + 124 - *y;

   print g;
   return 0;
}
