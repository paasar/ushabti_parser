# Ushabti parser

Algorithm to recognize different ushabti features from a screen capture from Spelunky 2 game.

Very much a work in progress.

I got this idea while playing Spelunky 2 where I found this room full of ushabti figurines.
This probably doesn't help me solve the mystery in the game, but I wanted to see what it would
take to recognize shapes and symbols from limited possibilities (three faces, five symbols, handful of colors).

Input:

![Input image](/images/ushabti_1_small.jpg)

Output image (click to open in full resolution):

![Output image](/images/output.png)

And output text:

```
Doing some image magic!
Dimensions (1052, 1066), ColorType Rgb8
Ushabti at [260, 480, 390, 650]
Bigger than previous ushabti at [260, 470, 390, 660]
Found a symbol at 290, 596, 360, 626
Symbol (probably) is: "Snake"
```

## TODO

I most likely won't develop this any further, but here are some things still left to do:

- Improve the accuracy of symbol detection
- Details step to recognise different faces
- Add other colors besides green
