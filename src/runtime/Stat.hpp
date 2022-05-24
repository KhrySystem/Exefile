#ifndef STAT_HPP
#define STAT_HPP

#include "lib.hpp"

struct Stat 
{
	Stat(std::string name) {

	}

	uint64_t ID;
	uint64_t value;
	uint64_t hits;
	uint8_t EQUATION_COEFFICIENT;
};

#endif