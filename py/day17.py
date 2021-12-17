import numpy as np
 
def part_1():
  #target area: x=150..193, y=-136..-86
  x_min = 150
  x_max = 193
  y_min = -136
  y_max = -86
  total = 0
  best_y = 0
  for sx in range(0,x_max+1):
    for sy in range(y_min,1000):
      x = 0
      y = 0
      x_vel = sx
      y_vel = sy
      max_y = 0
      while x <= x_max and y >= y_min:
        x_vel,y_vel,x,y = step(x_vel,y_vel,x,y)
        if y>max_y:
          max_y = y
        if x >= x_min and x <= x_max and y>=y_min and y<=y_max:
          total+=1
          if max_y > best_y:
            best_y = max_y
          break
        if x_vel == 0 and x < x_min:
          break
  print(best_y)
  print(total)
 
#take one step
def step(x_vel,y_vel,x,y):
  x+=x_vel
  y+=y_vel
  if x_vel > 0:
    x_vel -= 1
  elif x_vel < 0:
    x_vel += 1
  y_vel -= 1
  return x_vel,y_vel,x,y
 
part_1()