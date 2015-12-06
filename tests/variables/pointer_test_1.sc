// 2;
int main(int arg) {
   int x = 123;
   int* y = &x;

   int g = 100 + 24 - *y;
   g = g + 124 - *y;

   print g;
   return 0;
}
