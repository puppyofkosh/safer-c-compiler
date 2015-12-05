ERROR pointer
fn int main(int arg) {
   // Something you should not be allowed
   // to do:

   let pointer(int) p;
   
   if 3 == 3 {
      let int y = 5;

      // Should not be allowed to do this
      p = &y;
   }

   let int z = 3;

   print *p;

   return 0;
}
