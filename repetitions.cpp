#include<bits/stdc++.h>
using namespace std;
int main(){
    string input;
    cin>>input;
    int n=input.size();
    int curr=1;
    int ans=1;
    for(int i=1;i<n;i++){
        if(input[i]==input[i-1]) curr++;
        else {
            ans = max(ans,curr);
            curr=1;
        }
    }
    ans=max(ans,curr);//final streak
    cout<<ans;

    return 0;
}