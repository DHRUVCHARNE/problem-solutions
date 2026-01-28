/*
Many well-known cryptographic operations require modular exponentiation. That is, given integers x,
y and n, compute x^y mod n. In this question, you are tasked to program an efficient way to execute
this calculation
*/
#include <bits/stdc++.h>
using namespace std;
class Solution
{
public:
   static int modExp(int x, int y, int n)
   {
      int res = 1;
      x %= n;
      while (y > 0)
      {
         if (y & 1)
            res = (res * x) % n;
         x = (x * x) % n;
         y >>= 1;
      }
      return res;
   }
};
int main()
{
   ios::sync_with_stdio(false);
   cin.tie(NULL);
   int c;
   cin >> c;
   while (c--)
   {
      int x, y, n;
      cin >> x >> y >> n;
      cout << Solution::modExp(x, y, n) << "\n";
   }
   int dummy;
   cin >> dummy;

   return 0;
}