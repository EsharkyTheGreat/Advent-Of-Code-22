#include <bits/stdc++.h>
#include <iostream>

using namespace std;
void solve(ifstream &myfile) {
	string line;
	int curr_sum = 0;
	int max_sum = 0;
	while(getline(myfile,line)){
		if (line != ""){
			int num = stoi(line);
			curr_sum += num;
		}else{
			max_sum = max(max_sum,curr_sum);
			curr_sum = 0;	
		}
	}
	if (curr_sum != 0) {
		max_sum = max(curr_sum,max_sum);
	}
	cout << max_sum;
}
int main(){
	ios_base::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);
	ifstream myfile("input.txt");
	if (myfile.is_open()){
		solve(myfile);
		myfile.close();
	}
	return 0;
}
