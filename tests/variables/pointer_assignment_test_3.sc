// 6
// Make sure when doing a pointer deref, we copy the right amount of things
fn int main(int arg) {
   let char x = 1;
   let char y = 2;
   let char z = 3;
   let char w = 4;

   let pointer(char) p = &w;
   // Should only change value of w if we did this correctly
   *p = 23;

   print z + x + y;
}
