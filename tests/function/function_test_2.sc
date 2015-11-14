// 11
fn getValue(arg) {
   let v = 11;
   let y = 6;
   if v == y {
      let z = 123;
      print z;
      return z;
   }

   print v;   
   return v;
}

fn main(arg) {
   call(getValue, 0) ;
   return 0;
}