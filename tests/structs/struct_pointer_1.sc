// 600
struct MyStruct {
       char a;
       int x;
       char b;
}

int set300(MyStruct* p) {
   (*p).x = 300;

   return 0;
}

int main(int arg) {
   MyStruct s;

   // We want to make sure these values don't get overwritten when we
   // write to x   
   s.a = 200;
   s.b = 50;

   MyStruct* p = &s;

   set300(p);
   
   // At the same time when we write to s.a, we shouldn't overwrite x or b
   s.a = s.a + 50;

   print s.x + s.a + s.b;

   return 0;
}
