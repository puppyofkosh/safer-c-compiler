// 21
struct MyStruct {
       int x;
       pointer(int) y;
       char z;
}

fn int main(int arg) {
   let MyStruct s;

   let int x = 5;
   let char y = 10;

   s.x = 11;

   print s.x + y;
   return 0;
}
