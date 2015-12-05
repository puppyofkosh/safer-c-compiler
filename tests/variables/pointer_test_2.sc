// 2
int main(int arg) {
   let int x = 1;
   let pointer(int) y = &x;

   x = 2;

   print *y;
}
