// 202
fn int main(int arg) {
   let int x = 101;
   let char y = 255;
   let int z = 101;
   
   // Make sure that by changing y we don't accidently change x or z
   y = y + 15;

   print x + z;
   return 0;
}