// 25
int main(int arg)
{
        int tot = 0;
        int x = 0;
        while x < 5 {
          int y = 0;
          while y < 5 {
            tot = tot + 1;
            y = y + 1;
          }
          x = x + 1;
        }
        print tot;
}
