#ifndef PASSIVE_ABILITY_HPP
#define PASSIVE_ABILITY_HPP

#include "Ability.hpp"
#include "lib.hpp"

struct PassiveAbility : Ability
{
	uint64_t boost;
};


#endif