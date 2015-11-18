// 5;
fn f1(arg) {
   call(f2, arg + 1) ;
   return 0;
}

fn f2(arg) {
   print arg;
   return 0;
}

fn main(arg) {
   call(f1, 4) ;
   return 0;
}
