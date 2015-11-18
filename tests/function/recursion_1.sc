// 10
fn printuptoten(arg) {
   if arg == 10 {
      print arg;
   }
   if arg != 10 {
      print arg;
      call(printuptoten, arg + 1) ;
   }
   return 0;
}

fn main(arg) {
   call(printuptoten, 1) ;
   return 0;
}
