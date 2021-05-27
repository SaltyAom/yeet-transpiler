#include <stdio.h>

using namespace std;

int main() {
    printf("Read character array ");

    char message[127];
    scanf("%s", &message);

    printf("%s", message);
}