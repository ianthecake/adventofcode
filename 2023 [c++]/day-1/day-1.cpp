#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;

// TASK 01
int get_calibration_sum(string input) {
  fstream input_file;
  int calibration_sum = 0;

  input_file.open(input, ios::in);

  if (input_file.is_open()) {
    string line;

    while (getline(input_file, line)) {
      vector<int> line_values;
      for (char &c : line) {
        if (isdigit(c)) {
          int val = (int)c - '0';
          line_values.push_back(val);
        }
      }
      calibration_sum += line_values.front() * 10 + line_values.back();
    }

    input_file.close();
  }
  return calibration_sum;
}

// TASK 02
int get_real_calibration_sum(string input) {
  fstream input_file;
  int calibration_sum = 0;

  input_file.open(input, ios::in);

  if (input_file.is_open()) {
    string line;
    map<string, int> number_hashmap = {
        {"zero", 0}, {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4},
        {"five", 5}, {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9},
        {"0", 0},    {"1", 1},   {"2", 2},     {"3", 3},     {"4", 4},
        {"5", 5},    {"6", 6},   {"7", 7},     {"8", 8},     {"9", 9}};

    while (getline(input_file, line)) {
      vector<pair<int, int>> matches;

      for (size_t i = 0; i < line.size(); ++i) {
        for (const auto &entry : number_hashmap) {
          const string &word = entry.first;
          if (line.substr(i, word.length()) == word) {
            matches.push_back({(int)i, entry.second});
          }
        }
      }

      if (!matches.empty()) {
        int first = matches.front().second;
        int last = matches.back().second;
        calibration_sum += first * 10 + last;
      }
    }
    input_file.close();
  }
  return calibration_sum;
}

int main() {

  const string input = "input.txt";

  // TASK 0n_sum = get_real_calibration_sum(input);
  cout << "[TASK 02] " << "Real Calibration Sum: " << real_calibration_sum;
  // output: 53340

  return 0;
}
