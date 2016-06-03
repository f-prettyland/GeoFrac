# GeoFrac
Trying out Rust for the first time

For negative values use double-dash then a space before the value with no arguements after it. E.g. `geofrac gif 2 0.01 20 2 3 0.01 0.3750001200618655 -- -0.2166393884377127`

##Terminal Output with `term`
![](/res/TERMFractal.png)

##Greyscale Output with `-g`
![](/res/BWFractal.png)

##Colour Output
This uses a colouring function based off the cosine curve using:
![](/res/form.gif)
Where g represents a variable determined by the colour channel required and p is the number of passes required.
This gives an output like:
![](/res/COLFractal.png)

##Animated Output `gif`
Currently in production is an animated gif output, currently only outputs a collection of stills that can be conjoined. Sample output of `geofrac gif 2 0.01 20 2 3 0.1 0.275 0`
![](/res/anim.gif)
