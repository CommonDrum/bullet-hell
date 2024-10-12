import argparse
from PIL import Image, ImageDraw, ImageFont

def add_grid_to_image(image_path, output_path, columns, rows, tile_size, line_color=(0, 0, 0), text_color=(255, 0, 0)):
    img = Image.open(image_path)
    draw = ImageDraw.Draw(img)
    width, height = columns * tile_size, rows * tile_size

    for i in range(0, width, tile_size):
        draw.line((i, 0, i, height), fill=line_color, width=1)
    for j in range(0, height, tile_size):
        draw.line((0, j, width, j), fill=line_color, width=1)

    try:
        font = ImageFont.truetype("arial.ttf", 10)
    except IOError:
        font = ImageFont.load_default()

    counter = 1
    for j in range(rows):
        for i in range(columns):
            tile_x = i * tile_size
            tile_y = j * tile_size
            text_position = (tile_x + 5, tile_y + 5)
            draw.text(text_position, str(counter), font=font, fill=text_color)
            counter += 1

    img = img.crop((0, 0, width, height))
    img.save(output_path)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Add grid to an image with specified rows, columns, and tile size.')
    parser.add_argument('image_path', type=str, help='Path to the input image')
    parser.add_argument('output_path', type=str, help='Path to the output image (the original will not be replaced)')
    parser.add_argument('columns', type=int, help='Number of columns in the grid')
    parser.add_argument('rows', type=int, help='Number of rows in the grid')
    parser.add_argument('tile_size', type=int, help='Size of each tile in pixels')

    args = parser.parse_args()
    add_grid_to_image(args.image_path, args.output_path, args.columns, args.rows, args.tile_size)

