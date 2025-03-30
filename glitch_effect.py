from PIL import Image, ImageColor, ImageChops
import numpy as np
import random


# Must be an even number for CRT scan to work right
FRAMES = 10
# Designed to work with images that have a lot of transparency (e.g. text on empty BG)
INPUT = "input/flowers.png"
OUTPUT = "output/flowers.gif"


def sqrand(range: tuple[int, int]) -> int:
    return round((random.random() ** 2) * (range[1] - range[0]) + range[0])


def stripe_fn(x: int, frame: int) -> float:
    return 0.3 * ((x//2 + frame) % 2 == 0)


def shift_rows(img: Image, max_delta: int, run_range: tuple[int, int]) -> Image:
    np_image = np.array(img)
    height, _, _ = np_image.shape
    run, offset = sqrand(run_range), random.randint(-max_delta, max_delta)

    for y in range(height):
        np_image[y] = np.roll(np_image[y], offset, axis=0)
        run -= 1
        if run <= 0:
            run, offset = sqrand(run_range), random.randint(-max_delta, max_delta)

    return Image.fromarray(np_image)


def fill(img: Image, color: ImageColor) -> Image:
    filled = Image.new("RGBA", img.size, color + (255,))
    filled.putalpha(img.getchannel("A"))
    return filled


def shift(img: Image, delta: int) -> Image:
    return ImageChops.offset(img, delta, 0)


def apply_stripes(img: Image, frame: int) -> Image:
    alphas = np.array(img.getchannel("A"))

    stripes = Image.new("RGBA", img.size, (0, 0, 0, 255))
    for y in range(alphas.shape[0]):
        alphas[y] = np.round(alphas[y] * stripe_fn(y, frame))
    stripes.putalpha(Image.fromarray(alphas, mode="L"))

    img.paste(stripes, (0, 0), mask=stripes)


def generate_frame(seq: int) -> Image:
    im = Image.open(INPUT)

    base = shift_rows(im, 3, (2, 15))
    red = shift(fill(base, ImageColor.getrgb("#F55D3E")), 4)
    blue = shift(fill(base, ImageColor.getrgb("#63CCCA")), -4)

    final = red
    final.paste(blue, (0, 0), mask=blue)
    final.paste(base, (0, 0), mask=base)
    apply_stripes(final, seq)
    return final


if __name__ == "__main__":
    images = [generate_frame(i) for i in range(FRAMES)]
    images[0].save(
        OUTPUT,
        save_all=True,
        append_images=images[1:],
        optimize=True,
        duration=0.14,
        loop=0,
        disposal=2,
    )
