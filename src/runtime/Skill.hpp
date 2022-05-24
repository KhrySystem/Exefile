#ifndef SKILL_HPP
#define SKILL_HPP

#include "lib.hpp"

struct Skill 
{
	Skill(std::string name) {
		this->name = name;
		loadData();
	}

	void loadData() {
		std::ifstream file(name + ".skillData", std::ios::in | std::ios::binary);
		file >> ID;
		file >> *requirements;
		file >> requiredFor;
	}

	std::string name;
	uint64_t ID;
	uint64_t *requirements;
	uint64_t *requiredFor;

	void saveToFile() {
		std::ofstream file(name + ".skillData", std::ios::out | std::ios::trunc | std::ios::binary);
		file << ID << std::endl;
		file << *requirements << std::endl;
		file << *requiredFor << std::endl;
	}
};

#endif