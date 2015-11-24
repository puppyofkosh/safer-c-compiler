// 99

// This test can expose errors if our compiler mismanages the stack
// For it to pass, we must allocate the right amount of stack space within a block,
// and free that same amount when the block is over.
fn main(arg) {
   let int x = 43;
   let int y = 33;
   let int z = 23;
   

   // We will allocate some stack space for the variables w and q
   if x != y
   {
        let int w = 3;
        let char q = 4;
        print w;
   }
   // Will free that stack space

   // If we improperly freed the stack space, then this variable declaration will
   // mess up the value of x or y or z
   let int h = 555;

   print (x + y) + z;
   return 0;
}
