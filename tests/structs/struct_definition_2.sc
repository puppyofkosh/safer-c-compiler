// 35
struct MyStruct {
       char a;
       int x;
       char b;
}

int main(int arg) {
   let MyStruct s;

   s.x = 5;
   s.a = 10;
   s.b = 20;

   print s.x;
   print s.a;
   print s.b;

   print s.x + s.a + s.b;

   return 0;
}
