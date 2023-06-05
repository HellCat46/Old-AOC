#include <iostream>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

struct Elf {
    int position;
    long int calories;
};
vector<Elf> getData(const string&);
Elf getHighest(vector<Elf>);

int main() {

    vector<Elf> Elves = getData("../data/calories.txt");
    Elf elf = getHighest(Elves);
    Elves.clear();
    cout<<" Elf with "<<elf.calories<<" Calories worth of food "<<" at position "<<elf.position;

    return 0;
}

vector<Elf> getData(const string& path ){
    fstream s(path, ios::in);
    if(!s.good()) {
        cout<<"Failed to open the file";
        abort();
    }

    vector<Elf> Elves;
    {
        Elf elf = {
                0,
                0
        };

        int i = 0;
        while (!s.eof()) {
            char data[50];
            s.getline(data, 49);
            if (data[0] != 0) {
                elf.calories += stoi(data);
            } else {
                elf.position = i;
                Elves.push_back(elf);
                elf.calories = 0;
                i++;
            }

        }
    }
    s.close();
    return Elves;
}

Elf getHighest(vector<Elf> Elves){
    Elf selected_elf = {0,0};
    for(auto & elf : Elves){
        if(selected_elf.calories < elf.calories){
            selected_elf = {
                    elf.position,
                    elf.calories
            };
        }
    }
    Elves.clear();
    return selected_elf;
}
