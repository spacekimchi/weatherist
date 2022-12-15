## How to remove data from netCDF files

Create netCDF out.nc containing all variables from file in.nc. Restrict the dimensions of these variables to a hyperslab. The specified hyperslab is: the fifth value in dimension time; the half-open range lat > 0. in coordinate lat; the half-open range lon < 330. in coordinate lon; the closed interval 0.3 < band < 0.5 in coordinate band; and cross-section closest to 1000. in coordinate lev. Note that limits applied to coordinate values are specified with a decimal point, and limits applied to dimension indices do not have a decimal point See Hyperslabs.

`ncks -d time,5 -d lat,,0.0 -d lon,330.0, -d band,0.3,0.5 -d lev,1000.0 in.nc out.nc`

Assume the domain of the monotonically increasing longitude coordinate lon is 0 < lon < 360. Here, lon is an example of a wrapped coordinate. ncks will extract a hyperslab which crosses the Greenwich meridian simply by specifying the westernmost longitude as min and the easternmost longitude as max, as follows:

`ncks -d lon,260.0,45.0 in.nc out.nc`
