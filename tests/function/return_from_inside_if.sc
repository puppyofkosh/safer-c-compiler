// 5 6 0 1
// Make sure our stack doesn't get messed up by returing from within an inner block
int is_2(int arg) {
    if arg == 2 {
        int garbage;
        return 1;
    } else {
        return 0;
    }
}


int main(int arg) {
    // Make sure these values stay the same 
    int x = 5;
    int y = 6;
    
    int is_1_2 = is_2(1);
    int is_2_2 = is_2(2);
    
    printf("%d %d %d %d\n", x, y, is_1_2, is_2_2);
    fflush(0);
    
    return 0;
}
