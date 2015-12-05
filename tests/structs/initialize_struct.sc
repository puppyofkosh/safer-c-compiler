// ERROR typechecker
struct A {
       int x;
       int y;
}

int main(int arg) {
   A a;
   a.x = 5;
   a.y = 5;

   // not allowed
   A b = a;

   return 0;
}
