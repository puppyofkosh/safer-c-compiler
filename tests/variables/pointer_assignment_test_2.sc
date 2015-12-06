// 5
int setFive(int* arg) {
   *arg = 5;
   return 0;
}

int main(int arg) {
   int x = 11;
   setFive(&x);

   print x;
   return 0;
}
