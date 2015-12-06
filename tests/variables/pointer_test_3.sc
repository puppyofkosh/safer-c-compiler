// 2
int get_val(int* arg) {
   return *arg;
}

int main(int arg) {
   int x = 2;
   print get_val(&x);
   return 0;
}
