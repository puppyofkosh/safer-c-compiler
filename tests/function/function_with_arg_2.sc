// 21
int fun(int x) {
   if x == 21 {
      print x;
      return x;
   }

   int y = 35;
   return (x + y) - 50;
}

int main(int arg) {
   fun(21);
   return 0;
}
