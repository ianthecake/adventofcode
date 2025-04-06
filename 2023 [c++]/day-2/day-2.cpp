#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <string>
#include <vector>
using namespace std;

int get_line_id(string line) {
  int id_position = line.find_first_of(' ') + 1;
  string id_string = line.substr(id_position);
  int id = std::stoi(id_string);

  return id;
}

bool check_valid_game(map<string, int> color_counts,
                      map<string, int> max_counts) {
  for (const auto &entry : color_counts) {
    const string &color = entry.first;

    if (entry.second > max_counts[color]) {
      return false;
    }
  }
  return true;
}

vector<string> split_string(string str, char delimiter) {
  vector<string> substrings;
  vector<int> delimiter_positions;
  size_t start = 0;
  size_t end;

  while ((end = str.find(delimiter, start)) != string::npos) {
    substrings.push_back(str.substr(start, end - start));
    start = end + 1;
  }

  substrings.push_back(str.substr(start));
  return substrings;
}

map<string, int> get_game_color_counts(string line) {
  vector<map<string, int>> game_color_maps;

  map<string, int> color_map = {{"red", 0}, {"green", 0}, {"blue", 0}};

  vector<string> game_strings = split_string(line, ';');

  for (string gs : game_strings) {
    vector<string> color_strings = split_string(gs, ',');
    for (string cs : color_strings) {
      cout << "[COLOR_STRING " << cs << "\n";

      smatch num_match;
      smatch color_match;
      regex color_regex("(red|blue|green)");
      regex number_regex("\\d+");

      string color;
      int count = 0;

      if (regex_search(cs, color_match, color_regex)) {
        color = color_match[1];
        cout << "found color: " << color << "\n";
      }

      if (regex_search(cs, num_match, number_regex)) {
        count = stoi(num_match[0]);
        cout << "color count: " << count << "\n";
      }

      if (!color.empty()) {
        if (color_map.find(color) == color_map.end() ||
            count > color_map[color]) {
          color_map[color] = count;
        }
      }
    }
  }

  return color_map;
}

// TASK 01
int get_color_counts(string input, map<string, int> max_counts) {
  fstream input_file;
  int id_sum = 0;

  input_file.open(input, ios::in);

  if (input_file.is_open()) {
    string line;

    while (getline(input_file, line)) {
      string id_string, games_string;
      size_t delimiter_position = line.find(':');

      cout << "[WHOLE LINE] " << line << "\n";
      if (delimiter_position != std::string::npos) {
        id_string = line.substr(0, delimiter_position);
        cout << "[ID_STRING] " << id_string << "\n";
        games_string = line.substr(delimiter_position + 1);
        cout << "[GAMES_STRING] " << games_string << "\n";
      }

      int line_id = get_line_id(id_string);
      cout << "[LINE_ID] " << line_id << "\n";
      map<string, int> game_color_map = get_game_color_counts(games_string);

      if (check_valid_game(game_color_map, max_counts)) {
        id_sum += line_id;
      }
    }
  }

  return id_sum;
}

int main() {
  string input = "_input.txt";
  map<string, int> max_counts = {{"red", 12}, {"green", 13}, {"blue", 14}};

  int id_sum = get_color_counts(input, max_counts);
  cout << "SUM OF ALL POSSIBLE IDs: " << id_sum;
  return 0;
}
