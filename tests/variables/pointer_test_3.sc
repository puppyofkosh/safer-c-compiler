// 2
fn getVal(arg) {
   return *arg;
}

fn main(arg) {
   let int x = 2;
   print call(getVal, &x);
   return 0;
}