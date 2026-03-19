; ModuleID = 'ohl'
source_filename = "ohl"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 99, ptr %x, align 4
  %x1 = load i32, ptr %x, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, i32 %x1)
  ret i32 0
}
