; ModuleID = 'ohl'
source_filename = "ohl"

define i32 @add(i32 %x, i32 %y) {
entry:
  %result = alloca i32, align 4
  %y2 = alloca i32, align 4
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  store i32 %y, ptr %y2, align 4
  %x3 = load i32, ptr %x1, align 4
  %y4 = load i32, ptr %y2, align 4
  %add = add i32 %x3, %y4
  store i32 %add, ptr %result, align 4
}
