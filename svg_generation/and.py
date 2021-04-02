import svgwrite as svg
from svgwrite.shapes import Rect, Circle

s = svg.Drawing('../resources/and_gate.svg', profile='tiny')
s.viewbox(0, 0, 50, 75)
s.add(Rect((0, 0), (50, 50), fill=svg.rgb(255, 255, 255)))
s.add(Circle((50, 25), 25, fill=svg.rgb(255, 255, 255)))
s.save()
