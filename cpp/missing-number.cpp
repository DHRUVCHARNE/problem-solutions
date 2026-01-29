#include <bits/stdc++.h>
using namespace std;
int main(){
    int n;
    cin>>n;
    int c=0;
    long long actual_sum=0,ideal_sum=0;
    for(int i=0;i<n-1;i++) {
        cin>>c;
        actual_sum+=c;
    }
    ideal_sum=(1LL*n*(n+1))/2;
    cout<<ideal_sum-actual_sum;
    return 0;
}