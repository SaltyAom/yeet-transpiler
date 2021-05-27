#include <stdio.h>

using namespace std;

int main() {
    printf("Read char array");

    char message[127];
    scanf("%s", &message);

    printf("%s", message);
}