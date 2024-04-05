# simulate the three body problem using Rust

Recording: https://youtube.com/live/jZoukWyNprg

## Background

The so-called "three body problem" is the problem of deciding whether the orbits of three celestial bodies
-- anything with positive mass, e.g. galaxies, planets, rocks or dust -- are affected by gravity are stable.

## Repository description

- `original/` the [original code] as a git subtree
- `orbits/` the code created in the [video tutorial]

[original code]: https://github.com/achristmascarl/three_body
[video tutorial]: https://youtube.com/live/jZoukWyNprg

## Derivatives

- https://github.com/ollej/macroquad-three-bodies - uses Macroquad to create a simulation that runs in the browser

## Other resources

- the REBOUND library is probably the best implementation of n-body physics https://github.com/hannorein/rebound
- "Collisionless periodic orbits in the free-fall three-body problem". A paper which has tables of starting positions that generate stable orbits. https://arxiv.org/pdf/1805.07980.pdf
- A wonderful fediverse bot that creates lovely videos, with music (its source code is open source and much more realistic than ours) https://botsin.space/@ThreeBodyBot
- A HN thread that's quite interesting (and where I found the original) https://news.ycombinator.com/item?id=39909123






