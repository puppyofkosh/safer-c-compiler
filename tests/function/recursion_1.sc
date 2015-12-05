// 10
fn int printuptoten(int arg) {
   if arg == 10 {
      print arg;
   }
   if arg != 10 {
      print arg;
      printuptoten(arg+1);
   }
   return 0;
}

fn int main(int arg) {
   printuptoten(1);
   return 0;
}
