#ifndef SKILL_STAT_BOOST_HPP
#define SKILL_STAT_BOOST_HPP

#include "lib.hpp"
#include "Skill.hpp"
#include "Stat.hpp"

struct SkillStatBoost : Skill
{
	SkillStatBoost(std::string name, std::string statName) : Skill(name)
	{
		stat = Stat(statName);
	}

	Stat stat;

	void saveToFile() 
	{

	}
};

#endif