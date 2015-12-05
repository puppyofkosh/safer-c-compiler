// 2
int main(int arg) {
   let int x = 10;
   let pointer(int) y = &x;

   *y = 2;

   print x;
   return 0;
}
