// 21
struct MyStruct {
       int x;
       pointer(int) y;
       char z;
}

int main(int arg) {
   MyStruct s;

   int x = 5;
   char y = 10;

   s.x = 11;

   print s.x + y;
   return 0;
}
