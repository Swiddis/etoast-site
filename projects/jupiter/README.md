# Project Jupiter

This is an exploration in navigating the [SPICE dataset](https://github.com/openmm/spice-dataset).

In `kernels/` you need to navigate the NAIF generic kernels and retrieve:
- All the `jup*.bsp` files at https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/satellites/
- `naif0012.tls` at https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/
- `pck00011.tpc` and `gm_de_440.tpc` at https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/

The data was accessed on Feb 25 at 19:10 PST. If you want to follow along, you
can validate your data matches mine with `sha256sum -c kernels.sha256sum`.
