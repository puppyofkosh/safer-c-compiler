// 1

// Chars are just 1 byte, so their max value is 255. If we store a number > 255
// in a char, only the smallest bits will be saved in the char
fn int main(int arg) {
   let int x = 257;
   let char y = x;
   print y;
   return 0;
}
