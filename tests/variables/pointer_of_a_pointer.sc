// 12
int main(int arg) {
  int a = 12;
  int *b = &a;
  int** c = &b;
  int *d = *c;
  print *d;
}
