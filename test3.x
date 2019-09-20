strlen = core["string"]["strlen"]
concat = core["string"]["concat"]

print = std["io"]["print"]
println = std["io"]["println"]



physics = dict()


class physics["Point"] { fn new(self,x,y) { self.x = x
self.y = y
self }
fn goto(self,x,y) { self.x = x
self.y = y
self }
fn move(self,x,y) { self.x = add(self.x,x)
self.y = add(self.y,y)
self }
fn get_x(self) { self.x }
fn get_y(self) { self.y } }
True = fn(a) { fn(b) { a } }
False = fn(a) { fn(b) { b } }
If = fn(c) { fn(a) { fn(b) { (c(a))(b) } } }
println(((If(True))(5))(6))
x = 1
y = 1
println((((physics["Point"]()).new(1,2)).move(2,1)).move(x,y))
