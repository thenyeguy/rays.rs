# Rays

A Monte-Carlo Path Tracer in Rust. Features include:

* GGX microfacet material model for diffuse reflections and refractions.
* Importance sampling of rays for faster convergence.
* BVH object hierarchy for faster intersection lookup.
* Scene loader for `.obj` and `.mat` files.
* Optional integrated profiling and statistics counting.

## Reference

* [Microfacet Models for Refraction through Rough Surfaces](http://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf)
* [Importance Sampling techniques for GGX with Smith Masking-Shadowing](https://schuttejoe.github.io/post/ggximportancesamplingpart1/)
* [McGuire Computer Graphics Archive](https://casual-effects.com/data/)
