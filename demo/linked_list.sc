
struct Node {
    int val;
    Node* next;
}

int main(int arg) {
    printf("Instructions:\n");
    printf("a [num] - add a number to the front\n");
    printf("d [num] - delete the number if it exists\n");
    printf("p - print the numbers\n");
    printf("e - end the program\n");

    // For inputs
    char c;
    int x;

    Node *head = 0;
    Node *Null = 0;

    int flag = 1;
    while (flag) {
        scanf("%c", &c);
        if (c == 'a') {
            scanf("%d", &x);
            Node *nod = allocate(8);
            (*nod).val = x;
            (*nod).next = head;
            head = nod;
        }

        if (c == 'd') {
            scanf("%d", &x);
            Node *iter = head;
            Node *prev = 0;
            int flag2 = (head != Null);
            while (flag2 == 1) {
                if ((*iter).val == x) {
                    flag2 = 0;
                    if (iter == head) {
                        head = (*iter).next;
                    } else {
                        (*prev).next = (*iter).next;
                    }
                }

                prev = iter;
                iter = (*iter).next;
                if (iter == Null) { flag2 = 0; }
            } 
        }

        if (c == 'e') { flag = 0; }

        if (c == 'p') {
            Node *iter = head;
            while (iter != Null) {
                printf("%d ", (*iter).val);
                iter = (*iter).next;
            } 
            printf("\n");
        }

    }
    printf("Goodbye\n");
    return 0;
}
