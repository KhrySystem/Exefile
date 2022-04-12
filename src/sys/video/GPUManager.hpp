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

            void updateGPUList();

            int rateDeviceSutability(VkPhysicalDevice device);

            ~GPUManager();

        private:
            static inline std::vector<VkPhysicalDevice> devices;
            static inline std::vector<int> scores;
    };
};

#endif