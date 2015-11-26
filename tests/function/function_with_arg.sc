// 23
fn int func(int arg) {
   let int x = arg + 1;
   print x;
   return x;
}

fn int main(int arg) {
   call(func, 22);
   return 0;
}