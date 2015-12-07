// Quick sort
int qs(int left, int right, int *arr) {
    int i = left;
    int j = right;
    int mid = (left + right) / 2;
    while (i < j) {
        while ( *(arr + i*4) < *(arr + mid*4)) { i = i + 1; }
        while ( *(arr + j*4) > *(arr + mid*4)) { j = j - 1; }
        if (i <= j) {
            int tmp = *(arr + i*4);
            *(arr + i*4) = *(arr + j*4);
            *(arr + j*4) = tmp;
            i = i + 1;
            j = j - 1;
        }
    }
    if (left < j) { qs(left, j, arr); }
    if (i < right) { qs(i, right, arr); }
    return 0;
}

int main(int arg) {
    int n;
    printf("The length of the list?:");
    scanf("%d", &n); 
    int i = 0;
    int *arr = allocate(4*n);
    while (i < n) {
        scanf("%d", arr+(i*4));
        i = i + 1; 
    }
    qs(0, n-1, arr);
    i = 0;
    while (i < n) {
        printf("%d ", *(arr+i*4));
        i = i + 1;
    }
    printf("\n");
    return 0;
}

