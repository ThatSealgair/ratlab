# Installation script for ratlab
# 
# First step is to clone the compiler repository:
git clone https://github.com/ThatSealgair/ratlab.git

# Move into and compile the build file
cd ./ratlab/
cargo build

# Move the compiler into the system binaries
sudo cp target/debug/ratlab /usr/bin/ratlab

# Move out of the repo and remove
cd ../
rm -rf ./ratlab/
