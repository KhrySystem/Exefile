#ifndef MODEL_HPP
#define MODEL_HPP

#include <lib.hpp>

namespace Dragon {
    class Model {
        public:
            /*
                Called when the model is created.
                Input
                file: file path to load model from.
            */
            Model(std::string file) {

            }

            /*
                Method to move model around the gameSpace.
                x, y, z: game coordinates as the rotate point
            */
            void moveTo(float x, float y, float z) {

            }
            /*
                Called when the model is destroyed.
                x, y, z: rotation from the previous rotation.
            */
            void rotate(float x, float y, float z) {

            }

            /*
                Called when the model is destroyed.
            */
            ~Model() {

            }

        private:
            struct
    };
};

#endif