#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main() {
  // 메모리를 확보한 뒤 문자열을 복사 --- (*1)
  char *g1 = (char *)malloc(100);
  strcpy(g1, "온화한 마음은 몸에 좋다.");
  // 변수 g2에 g1을 대입 --- (*2)
  char *g2 = g1;
  // g2의 내용을 출력 --- (*3)
  printf("%s\n", g2);
  // 메모리 해제 --- (*4)
  free(g2);
  // 실수로 다음을 실행하면 메모리 이중 해제
  // free(g1); 
  return 0;
}