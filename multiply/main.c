#include "multiply.h"
#include "stdio.h"

int main() {
  int a = 25;
  int b = 30;
  int c = 35;
  int d = 54;
  int e = 654;

  int result = multiply(a,b,c,d,e);
  printf("%d\n", result);
  return 0;
}
