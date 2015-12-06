// 2
int main(int arg) {
   int x = 10;
   int* y = &x;

   *y = 2;

   print x;
   return 0;
}
