// 21
fn int fun(int x) {
   if x == 21 {
      print x;
      return x;
   }
   
   let int y = 35;
   return (x + y) - 50;
}

fn int main(int arg) {
   call(fun, 21) ;
   return 0;
}
