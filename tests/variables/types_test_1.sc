// 1;
int main(int arg) {
   int x = 53;
   char y = 53;
   int z = 12345;

   // When comparing x with y, we should only load 1 byte
   // out of where y is stored to get its value. If that doesn't
   // happen, we'll load 1 byte from y and 3 bytes from z,
   // and the test will fail
   if x == y {
      print 1;
   }  
   return 0;
}
