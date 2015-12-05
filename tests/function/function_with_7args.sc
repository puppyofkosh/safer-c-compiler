// 1111111
int func(int arg1, int arg2, int arg3, int arg4, int arg5, int arg6, int arg7) {
   int x = arg1 + arg2 + arg3 + arg4 + arg5 + arg6 + arg7;
   print x;
   return x;
}

int main(int arg) {
   func(1, 10, 100, 1000, 10000, 100000, 1000000);
   return 0;
}
