// 202
int main(int arg) {
   int x = 101;
   char y = 255;
   int z = 101;
   
   // Make sure that by changing y we don't accidently change x or z
   y = y + 15;

   print x + z;
   return 0;
}
