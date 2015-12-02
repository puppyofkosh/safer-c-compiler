// 123
struct MyStruct {
       int x;
}

fn int main(int arg) {
   let MyStruct s;
   let Pointer(MyStruct) p = &s;

   print (*p).x;
}
