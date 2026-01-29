#include<bits/stdc++.h>
using namespace std;
class Solution {
    public:
    static void printBeautifulPermutation(int n){
        if(n==2 | n==3) {
            cout<<"NO SOLUTION";
            return ;
        }
        //Print even numbers first then odd
        for(int i=2;i<=n;i+=2) cout<<i<<" ";
        for(int i=1;i<=n;i+=2) cout<<i<<" ";
    }
};

int main(){
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    int n;
    cin>>n;
    Solution::printBeautifulPermutation(n);
    return 0;
}