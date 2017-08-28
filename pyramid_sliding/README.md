[/r/dailyprogrammer - Pyramid Sliding](https://www.reddit.com/r/dailyprogrammer/comments/6vi9ro/170823_challenge_328_intermediate_pyramid_sliding/)

Test data can be found in the '/pyramids/' folder.


First method I've tried is a greedy search: This method is not guarenteed to find the best search path.
It finds the optimal path for challenge1.pyramid, but not for the others. (find this in slide_down_greedily() in main.rs).

Second methods was depth-first-search: This method found the best solution for the first two, but was taking longer for the third challenge dataset that I gave up. (find this in slide_down_depth_first() in main.rs).