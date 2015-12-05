// 99

// This test can expose errors if our compiler mismanages the stack
// For it to pass, we must allocate the right amount of stack space within a block,
// and free that same amount when the block is over.
int main(int arg) {
   int x = 43;
   int y = 33;
   int z = 23;
   

   // We will allocate some stack space for the variables w and q
   if x != y
   {
        int w = 3;
        char q = 4;
        print w;
   }
   // Will free that stack space

   // If we improperly freed the stack space, then this variable declaration will
   // mess up the value of x or y or z
   int h = 555;

   print (x + y) + z;
   return 0;
}
