#ifndef READ_FILE_HPP
#define READ_FILE_HPP

#include "lib.hpp"

std::string readFile(std::string fileName) 
{
	std::string out;
	std::string line;
	std::ifstream file;
	file.open(fileName);

	if (!file.is_open())
		return "";

	while (std::getline(file, line))
		out += line;

	return out;
}

bool writeFile(char* fileName, char* data)
{

}

#endif