# Rusty-Bedtime-Timer
A simple, short bedtime timer which incrementally blocks more blue light than traditional blue-filter programs, while making the screen still comfortable to look at. When bedtime is reached, the screen becomes somewhat reddish, making it intentionally uncomfortable to look at (although still usable). Makes you very tired at the end. Great for people who want to be consequential with fixing their sleep cycle. At the moment it is written for Linux only, using Rust and xsct. 

The change sequence in color temperature happens in the following order:

3 hours before bed: 4000K
2.5 hours before bed: 3000K
2 hours before bed: 2000K (still comfortable to look at, but slightly tiring)
At reaching bedtime: 1000K (very reddish, will make you want to go to sleep)

INSTALL:
To run on Ubuntu or a similar Linux distribution, you simply have to install xsct and either run rusty_bedtime_timer, or build it yourself with cargo.
