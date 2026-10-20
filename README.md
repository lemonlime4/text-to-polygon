# Text to desmos

Convert text to a desmos polygon. Check it out [here](https://lemonlime4.github.io/text-to-polygon).

### What is this for?

If you want to add text to your graph, you have several options:

- Point labels: probably the fastest option; however, positioning and sizing is hard to do, and only a few fonts are available (using latex commands).

- Parametrics ([SVG to Desmos](https://github.com/jared-hughes/svgToDesmos)): easy to position, change fonts, etc. You can also pass these through arbitrary transformations, such as warping.

- Polygons: same as parametrics, except they're pre-sampled. This avoids sampling artifacts and reduces lag for animations. However, you need to manually adjust the sample density to a desired level.

Compare animated [polygon](https://www.desmos.com/calculator/3g48l5gqxt) vs [parametric](https://www.desmos.com/calculator/gsmxpxp97g) text.

### Limitations

You can only convert one segment of text at a time. If you want line breaks or multiple font faces, you have to manually save the sampled points and advance widths, and combine them. Also, if you want styles like italic or bold, you have to load individual font files for those.

Also, for longer text segments, each glyph gets sampled and stored every time it appears, instead of sampling once and instancing at different locations. This might slow down the graph if you're doing lots of text.

More to come soon (or not?)
