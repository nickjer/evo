let w;
let board_idx = 0;
let columns;
let rows;
let isPaused = false;

const colors = [
  "aliceblue", "antiquewhite", "aqua", "aquamarine", "azure", "beige",
  "bisque", "black", "blanchedalmond", "blue", "blueviolet", "brown",
  "burlywood", "cadetblue", "chartreuse", "chocolate", "coral",
  "cornflowerblue", "cornsilk", "crimson", "cyan", "darkblue", "darkcyan",
  "darkgoldenrod", "darkgray", "darkgreen", "darkgrey", "darkkhaki",
  "darkmagenta", "darkolivegreen", "darkorange", "darkorchid", "darkred",
  "darksalmon", "darkseagreen", "darkslateblue", "darkslategray",
  "darkslategrey", "darkturquoise", "darkviolet", "deeppink", "deepskyblue",
  "dimgray", "dimgrey", "dodgerblue", "firebrick", "floralwhite",
  "forestgreen", "fuchsia", "gainsboro", "ghostwhite", "gold", "goldenrod",
  "gray", "green", "greenyellow", "grey", "honeydew", "hotpink", "indianred",
  "indigo", "ivory", "khaki", "lavender", "lavenderblush", "lawngreen",
  "lemonchiffon", "lightblue", "lightcoral", "lightcyan",
  "lightgoldenrodyellow", "lightgray", "lightgreen", "lightgrey", "lightpink",
  "lightsalmon", "lightseagreen", "lightskyblue", "lightslategray",
  "lightslategrey", "lightsteelblue", "lightyellow", "lime", "limegreen",
  "linen", "magenta", "maroon", "mediumaquamarine", "mediumblue",
  "mediumorchid", "mediumpurple", "mediumseagreen", "mediumslateblue",
  "mediumspringgreen", "mediumturquoise", "mediumvioletred", "midnightblue",
  "mintcream", "mistyrose", "moccasin", "navajowhite", "navy", "oldlace",
  "olive", "olivedrab", "orange", "orangered", "orchid", "palegoldenrod",
  "palegreen", "paleturquoise", "palevioletred", "papayawhip", "peachpuff",
  "peru", "pink", "plum", "powderblue", "purple", "rebeccapurple", "red",
  "rosybrown", "royalblue", "saddlebrown", "salmon", "sandybrown", "seagreen",
  "seashell", "sienna", "silver", "skyblue", "slateblue", "slategray",
  "slategrey", "snow", "springgreen", "steelblue", "tan", "teal", "thistle",
  "tomato", "transparent", "turquoise", "violet", "wheat", "whitesmoke",
  "yellow", "yellowgreen",
];

function setup() {
  columns = board["x_size"];
  rows = board["y_size"];
  w = 4;

  createCanvas(columns * w + 100, rows * w);

  frameRate(5);

  textSize(30);
  textAlign(CENTER, CENTER);
}

function keyPressed() {
  if (key === "p") {
    if (isPaused === true) {
      loop();
      isPaused = false;
    } else {
      noLoop();
      isPaused = true;
    }
  } else if (key === "h") {
    if (board_idx > 1) {
      board_idx -= 2;
      redraw();
    }
  } else if (key === "l") {
    redraw();
  }
}

function draw() {
  background(255);
  fill(0);
  text(board_idx, width - 50, 50);
  let board = tile_snapshots[board_idx];
  for ( let i = 0; i < columns; i++) {
    for ( let j = 0; j < rows; j++) {
      if (board[i][j] == 0) {
        fill(255);
      } else {
        let color = colors[(board[i][j] - 1) % colors.length];
        fill(color);
      }
      strokeWeight(1);
      stroke(0, 10);
      rect(i * w, j * w, w-1, w-1);
    }
  }
  board_idx++;
  if (board_idx >= tile_snapshots.length) {
    noLoop();
  }
}
