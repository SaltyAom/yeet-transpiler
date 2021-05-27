#include<stdio.h>

int main() {
    int a,b,i,j,x=0;
    scanf("%d %d", &a, &b);
    int m[a][b], n[a], o[b];
    for(i=0;i<a;i++) n[i]=0;
    for(i=0;i<b;i++) o[i]=0;
    for(i=0;i<a;i++) for(j=0;j<b;j++) scanf("%d", &m[i][j]);
    for(i=0;i<a;i++) for(j=0;j<b;j++) n[i]+=m[i][j],o[j]+=m[i][j]; 
    for(i=0;i<a;i++) for(int j=0;j<b;j++) if(n[i]+o[j]-m[i][j]>x) x=n[i]+o[j]-m[i][j];
    printf("%d",x);
    return 0;
}