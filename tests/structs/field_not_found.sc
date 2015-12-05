// ERROR typechecker
struct A {
       int x;
       int y;
}

int main(int arg) {
   let A a;

   a.x = 10;
   a.y = 20;
   a.z = 30;

   print a.x;

   return 0;
}