#include <bits/stdc++.h>
using namespace std;
/*
Calculate
R := B^P mod M
for large values of B, P, and M using an efficient algorithm. (That’s right, this problem has a time
dependency !!!.)
*/
int main(){
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    long long B,P;
    int M;
    while(cin>>B>>P>>M){
    if(M==1) cout<<0<<"\n";
    long long res=1;
    B=B%M;
    while(P>0){
        if(P&1) res= res*B%M;
        B=B*B%M;
        P>>=1;
    }
    cout<<res<<"\n";
}
    return 0;
}