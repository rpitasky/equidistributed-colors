Iterator over colors which are distributed evenly according to the equidistribution theorem. Attempts to make the next color look as different as possible from all of the previously generated colors. Compatible with the [`rgb`](https://docs.rs/rgb/latest/rgb/) crate. Possible uses include assigning colors to players, where the total number of players is unknown.

**This crate makes no guarantees about readability for people with color blindness or other visual impairments.**

Inspired by [this stack exchange post](https://gamedev.stackexchange.com/questions/46463/how-can-i-find-an-optimum-set-of-colors-for-10-players), using oklab to better predict the intensity.