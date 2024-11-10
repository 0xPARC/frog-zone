var Jimp = require("Jimp");
var fs = require("fs");

// Create two-dimensional pixels rgb array based on png image
Jimp.Jimp.read("./fhe-map01.png")
  .then((image) => {
    var width = image.bitmap.width;
    var height = image.bitmap.height;
    console.log(width, height);
    var pixels = {};
    for (var y = 0; y < height; y++) {
      for (var x = 0; x < width; x++) {
        var pixel = Jimp.intToRGBA(image.getPixelColor(x, y));
        var obj = {
          terrainType: "NONE",
        };
        if (pixel.r === 188 && pixel.g === 217 && pixel.b === 238) {
          obj = {
            terrainType: "ICE",
          };
        } else if (pixel.r === 63 && pixel.g === 175 && pixel.b === 228) {
          obj = {
            terrainType: "WATER",
          };
        } else if (pixel.r === 255 && pixel.g === 243 && pixel.b === 191) {
          obj = {
            terrainType: "SAND",
          };
        } else if (pixel.r === 126 && pixel.g === 67 && pixel.b === 40) {
          obj = {
            terrainType: "ROCK",
          };
        } else if (pixel.r === 104 && pixel.g === 190 && pixel.b === 100) {
          obj = {
            terrainType: "GRASS",
          };
        }
        pixels[`${x},${y}`] = obj;
      }
    }
    fs.writeFile("INPUT_DATA.json", JSON.stringify(pixels), "utf8", (err) => {
      if (err) {
        throw err;
      }
    });
  })
  .catch((err) => {
    throw err;
  });

// Create png image based on two-dimensional pixels rgb array
// fs.readFile("INPUT_DATA.json", "utf8", (err, file) => {
//   if (err) {
//     throw err;
//   }
//   var pixelsData = JSON.parse(file);
//   var pixels = pixelsData.data;
//   new Jimp(pixels[0].length, pixels.length, (err, image) => {
//     if (err) {
//       throw err;
//     }
//     pixels.forEach((rowPixels, y) => {
//       rowPixels.forEach((pixel, x) => {
//         var rgb = pixel.split(",");
//         var r = Number(rgb[0]);
//         var g = Number(rgb[1]);
//         var b = Number(rgb[2]);
//         var color = Jimp.rgbaToInt(r, g, b, 255);
//         image.setPixelColor(color, x, y);
//       });
//     });
//     image.write("OUTPUT_IMAGE.png", (err) => {
//       if (err) {
//         throw err;
//       }
//     });
//   });
// });
