// 5
int setFive(pointer(int) arg) {
   *arg = 5;
   return 0;
}

int main(int arg) {
   int x = 11;
   setFive(&x);

   print x;
   return 0;
}
