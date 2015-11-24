// 21
fn fun(x) {
   if x == 21 {
      print x;
      return x;
   }
   
   let int y = 35;
   return (x + y) - 50;
}

fn main(arg) {
   call(fun, 21) ;
   return 0;
}
