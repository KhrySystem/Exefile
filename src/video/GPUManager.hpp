#ifndef GPU_MANAGER_HPP
#define GPU_MANAGER_HPP

#include <VkInstanceReference.hpp>
#include <lib.hpp>

namespace Dragon {
    class GPUManager {
        public:
            GPUManager();

            VkPhysicalDevice getBestDevice();
            VkPhysicalDevice* getDevices();
            int* getScores();

            int rateDeviceSutability(VkPhysicalDevice device);

            ~GPUManager();

        private:
            static std::vector<VkPhysicalDevice> devices;
            static std::vector<int> scores;
    };
};

#endif