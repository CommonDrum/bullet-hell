import argparse
from PIL import Image, ImageDraw, ImageFont

def add_grid_to_image(image_path, output_path, tile_size, line_color=(0, 0, 0), text_color=(255, 0, 0)):
    img = Image.open(image_path)
    draw = ImageDraw.Draw(img)

    width, height = img.size
    columns = width // tile_size
    rows = height // tile_size

    new_height = height + 30  
    img_with_text = Image.new("RGBA", (width, new_height), (255, 255, 255, 0))
    img_with_text.paste(img, (0, 0))
    draw = ImageDraw.Draw(img_with_text)

    for i in range(0, width, tile_size):
        draw.line((i, 0, i, height), fill=line_color, width=1)
    for j in range(0, height, tile_size):
        draw.line((0, j, width, j), fill=line_color, width=1)

    font_size = int(tile_size * 0.5)  # Font size is set to 50% of the tile size
    try:
        font = ImageFont.truetype("arial.ttf", font_size)
    except IOError:
        font = ImageFont.load_default()

    counter = 1
    for j in range(rows):
        for i in range(columns):
            tile_x = i * tile_size
            tile_y = j * tile_size
            text = str(counter)

            text_bbox = draw.textbbox((0, 0), text, font=font)
            text_width = text_bbox[2] - text_bbox[0]
            text_height = text_bbox[3] - text_bbox[1]
            text_position = (
                tile_x + (tile_size - text_width) // 2,
                tile_y + (tile_size - text_height) // 2
            )

            draw.text(text_position, text, font=font, fill=text_color)
            counter += 1

    text = f"Rows: {rows}, Columns: {columns}"
    text_bbox = draw.textbbox((0, 0), text, font=font)
    text_width, text_height = text_bbox[2] - text_bbox[0], text_bbox[3] - text_bbox[1]
    text_position = ((width - text_width) // 2, height + 5)
    draw.text(text_position, text, font=font, fill=text_color)

    img_with_text.save(output_path)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Add grid to an image based on tile size and display row/column count at the bottom.')
    parser.add_argument('image_path', type=str, help='Path to the input image')
    parser.add_argument('output_path', type=str, help='Path to the output image (the original will not be replaced)')
    parser.add_argument('tile_size', type=int, help='Size of each tile in pixels')

    args = parser.parse_args()
    add_grid_to_image(args.image_path, args.output_path, args.tile_size)

