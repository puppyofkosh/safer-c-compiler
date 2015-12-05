// ERROR typechecker
struct A {
       int x;
       int y;
}

int main(int arg) {
   let A a;
   a.x = 5;
   a.y = 5;

   // not allowed
   let A b = a;
   
   return 0;
}
