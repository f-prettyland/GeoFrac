# GeoFrac
Trying out Rust for the first time

For negative values use double-dash then a space before the value with no arguements after it. E.g. `geofrac gif 4 0.01 20 2 3 0.01 0.3750001200618655 -- -0.2166393884377127`

## Terminal Output with `term`
![](/res/TERMFractal.png)

## Greyscale Output with `-g`
![](/res/BWFractal.png)

## Colour Output
This uses a colouring function based off the cosine curve using:
![](/res/form.gif)
Where g represents a variable determined by the colour channel required and p is the number of passes required.
This gives an output like:
![](/res/COLFractal.png)

## Animated Output `gif`
Currently in production is an animated gif output, currently only outputs a collection of stills that can be conjoined. 

Sample output of `geofrac gif 4 0.01 20 2 3 0.01 0.3750001200618655 0.2166393884377127` and `geofrac gif 4 0.01 20 2 3 0.1 0.275 0`


![](/res/anim.gif)
![](/res/anim1.gif)


##Todo
### Quick
- `Escape_radius` to be optional
- Allow user to input center and size for still generation
- Find out why cannot generate above iteration 54 in `geofrac gif 4 0.01 20 2 3 0.01 0.3750001200618655 0.2166393884377127`

### PREDICITVE:
If pooint lies within bounds of max iterations of previous one, probably will be max again. I.E.

### Parallelism
- crossbeam/Metal IO

#### BEHAVIOURIAL ALLOCATION
- Check threads using 
 + https://doc.rust-lang.org/book/benchmark-tests.html
 + https://llogiq.github.io/2015/06/16/bench.html
- if neighbour is one pass, likely that is is one pass (therefore trivial)
- Thread creation according to complexity


### Generics
- Fractal finder with custom fnc input 
	
### Experiments:
#### "Bad version"
Rounding to figures found before to save time
