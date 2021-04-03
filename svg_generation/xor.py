import svgwrite as svg
from svgwrite.shapes import Ellipse, Rect, Polygon

white = svg.rgb(255, 255, 255)
black = svg.rgb(0, 0, 0)
strokewidth = 1.25

s = svg.Drawing("../resources/xor_gate.svg", profile="tiny")
s.viewbox(0, 0, 100, 75)
s.add(
    Polygon(
        points=[
            (20 - strokewidth * 2, strokewidth),
            (30, strokewidth),
            (50, strokewidth + 5),
            (60, strokewidth + 10),
            (70, strokewidth + 15),
            (80, strokewidth + 20),
            (85, strokewidth + 25),
            (90, 75 / 2),
            (85, 75 - strokewidth - 25),
            (80, 75 - strokewidth - 20),
            (70, 75 - strokewidth - 15),
            (60, 75 - strokewidth - 10),
            (50, 75 - strokewidth - 5),
            (30, 75 - strokewidth),
            (20 - strokewidth * 2, 75 - strokewidth),
        ],
        fill=white,
        stroke=white,
    )
)

s.add(
    Ellipse(
        (10, 75 / 2),
        (25, 75 / 2 + strokewidth),
        stroke=white,
        stroke_width=strokewidth,
        fill=black,
    )
)
s.add(Rect((0, -10), (20 - strokewidth * 1.5, 100), stroke=black, fill=black))

s.add(
    Ellipse(
        (-5, 75 / 2),
        (25, 75 / 2 - strokewidth),
        stroke=white,
        stroke_width=strokewidth,
        fill_opacity = 0.0,
    )
)
s.save()
