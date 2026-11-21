#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <thread>
#include <chrono>
#include <cmath>

std::vector<std::string> splitLines(const std::string& text) {
    std::vector<std::string> lines;
    std::stringstream ss(text);
    std::string line;
    while (std::getline(ss, line)) {
        lines.push_back(line);
    }
    return lines;
}

void printVerticalGradient(const std::vector<std::string>& lines, int r1, int g1, int b1, int r2, int g2, int b2) {
    int totalLines = lines.size();

    std::cout << "\033[H"; 

    for (int i = 0; i < totalLines; ++i) {
        float ratio = (totalLines <= 1) ? 0.0f : static_cast<float>(i) / (totalLines - 1);
        int r = static_cast<int>(r1 + ratio * (r2 - r1));
        int g = static_cast<int>(g1 + ratio * (g2 - g1));
        int b = static_cast<int>(b1 + ratio * (b2 - b1));
        
        std::cout << "\033[38;2;" << r << ";" << g << ";" << b << "m" << lines[i] << "\033[0m\n";
    }
}

int main() {
    std::cout << "\033[2J\033[?25l";

    std::string heart = R"(⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⣶⣶⣦⠀⠀
⠀⠀⣠⣤⣤⣄⣀⣾⣿⠟⠛⠻⢿⣷⠀
⢰⣿⡿⠛⠙⠻⣿⣿⠁⠀⠀⠀⣶⢿⡇
⢿⣿⣇⠀⠀⠀⠈⠏⠀⠀⠀ C++&Rust&Nim
⠀⠻⣿⣷⣦⣤⣀⠀⠀⠀⠀⣾⡿⠃⠀
⠀⠀⠀⠀⠉⠉⠻⣿⣄⣴⣿⠟⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣿⡿⠟⠁⠀⠀⠀⠀)";

    auto lines = splitLines(heart);
    float phase = 0.0f;

    while (true) {
        int r1 = static_cast<int>(127 + 127 * std::sin(phase));
        int r2 = static_cast<int>(127 + 127 * std::cos(phase));
        int b1 = static_cast<int>(127 + 127 * std::cos(phase));
        int b2 = static_cast<int>(127 + 127 * std::cos(phase));
        
        printVerticalGradient(lines, r1, r2, b1, b2, r1, b2);
        
        phase += 0.08f;
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }

    return 0;
}
