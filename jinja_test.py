import jinja2

loader = jinja2.FileSystemLoader("templates")
env = jinja2.Environment(loader=loader)

# these all work
print(env.get_template("loop-things.txt.j2").render(
  things=[
    {'name': 'a'},
    {'name': 'b'}
  ]
))

print(env.get_template("static-things.txt.j2").render())