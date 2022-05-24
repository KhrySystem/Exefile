#ifndef ABILITY_HPP
#define ABILITY_HPP

#include "lib.hpp"
#include "ReadFromFile.hpp"

struct Ability 
{
	Ability(std::string name) {
		this->name = name;
		std::ifstream file(name + ".abilityData", std::ios::in | std::ios::binary);
		file >> level;
		file >> hits;
		file >> EQUATION_COEFFICIENT;
	}
	std::string name;
	uint64_t level, hits;
	uint8_t EQUATION_COEFFICIENT;
	void* onActivate;

	void saveToFile() {
		std::ofstream file(name + ".abilityData", std::ios::out | std::ios::trunc | std::ios::binary);
		file << level << std::endl;
		file << hits << std::endl;
		file << EQUATION_COEFFICIENT << std::endl;
	}
};

#endif