; ModuleID = 'ohl'
source_filename = "ohl"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca i8, align 1
  store i8 116, ptr %x, align 1
  %x1 = load i8, ptr %x, align 1
  %char_promote = zext i8 %x1 to i32
  %sub = sub i32 %char_promote, 122
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, i32 %sub)
  ret i32 0
}
