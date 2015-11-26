// 11
fn int getValue(int arg) {
   let int v = 11;
   let int y = 6;
   if v == y {
      let int z = 123;
      print z;
      return z;
   }

   print v;   
   return v;
}

fn int main(int arg) {
   call(getValue, 0) ;
   return 0;
}