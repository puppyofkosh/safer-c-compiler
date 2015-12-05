// 6
// Make sure when doing a pointer deref, we copy the right amount of things
int main(int arg) {
   char x = 1;
   char y = 2;
   char z = 3;
   char w = 4;

   pointer(char) p = &w;
   // Should only change value of w if we did this correctly
   *p = 23;

   print z + x + y;
}
