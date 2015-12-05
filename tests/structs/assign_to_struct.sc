// ERROR typechecker

struct A {
       int x;
       int y;
}

int main(int arg) {
   A a;
   a.x = 5;
   a.y = 6;


   A b;

   // Not allowed.
   b = a;

   return 0;
}
