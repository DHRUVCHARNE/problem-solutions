#include<bits/stdc++.h>
using namespace std;

class LRUCache {
    int capacity;
    //Doubly linked list most recent at front
    list<pair<int,int>> dll; //{key,value}
    //Map key-> iterator pointing to the list
    unordered_map<int,list<pair<int,int>>::iterator> cacheMap;
public:
    LRUCache(int capacity): capacity(capacity){}
    
    int get(int key) {
        if(cacheMap.find(key)==cacheMap.end()) return -1;
        //Move Accessed Node to front
        auto it = cacheMap[key];
        int value=it->second;
        dll.erase(it);
        dll.push_front({key,value});
        cacheMap[key]=dll.begin();
        return value;
    }
    
    void put(int key, int value) {
        //If key exists remove old posititon
        if(cacheMap.find(key)!=cacheMap.end()) dll.erase(cacheMap[key]);
        //Insert at front
        dll.push_front({key,value});
        cacheMap[key]=dll.begin();
        //Evict LRU if over capacity
        if(dll.size()>capacity){
            auto last=dll.back();
            cacheMap.erase(last.first);
            dll.pop_back();
        }
    }
};
