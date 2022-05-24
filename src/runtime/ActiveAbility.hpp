#ifndef ACTIVE_ABILITY_HPP
#define ACTIVE_ABILITY_HPP

#include "Ability.hpp"
#include "lib.hpp"

struct ActiveAbility : Ability
{
	double cooldown;
};

#endif