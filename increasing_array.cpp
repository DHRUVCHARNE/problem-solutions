#include <bits/stdc++.h>
using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    int n;
    cin>>n;
    vector<int> arr(n);
    for(int i=0;i<n;i++) cin>>arr[i];
    long long moves=0;
    for(int i=1;i<n;i++){
        if(arr[i]<arr[i-1]){
            moves+=(arr[i-1]-arr[i]);
            arr[i]=arr[i-1];
        }
    }
    cout<<moves;
    return 0;

}