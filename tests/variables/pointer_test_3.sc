// 2
int get_val(pointer(int) arg) {
   return *arg;
}

int main(int arg) {
   int x = 2;
   print get_val(&x);
   return 0;
}
