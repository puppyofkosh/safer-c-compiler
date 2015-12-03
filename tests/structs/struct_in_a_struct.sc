// 60
struct A {
       int x;
       int y;
}

struct B {
       A x;
       A y;
}

fn int main(int arg) {
   let B b;

   b.x.x = 10;
   b.x.y = 10;
   b.y.x = 20;
   b.y.y = 20;

   print b.x.x + b.x.y + b.y.x + b.y.y;
}
