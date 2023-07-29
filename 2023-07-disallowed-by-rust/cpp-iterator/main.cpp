#include <iostream>
#include <vector>

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};
    for (std::vector<int>::iterator it = vec.begin(); it != vec.end(); ++it) {
        vec.push_back(6); // This could invalidate the iterator
        std::cout << *it << std::endl;
    }
    return 0;
}

