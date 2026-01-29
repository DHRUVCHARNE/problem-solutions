#include <bits/stdc++.h>
using namespace std;
int main(){
    long long num;
    cin>>num;
    cout<<num<<" ";
    while(num!=1){
        if(num&1) num=num*3+1;
        else num>>=1;
        cout<<num<<" ";
    }

    return 0;
}