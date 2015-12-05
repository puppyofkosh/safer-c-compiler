// 10
// last character is a \n (which is 10 in ascii)
int main(int arg) {
   let pointer(char) p = "hallo thar\n";
   
   while *p != 0 {
         print *p;
         p = p + 1;
   }

   return 0;
}
