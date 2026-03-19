; ModuleID = 'ohl'
source_filename = "ohl"

@str = private unnamed_addr constant [13 x i8] c"hello world!\00", align 1
@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca ptr, align 8
  store ptr @str, ptr %x, align 8
  %x1 = load ptr, ptr %x, align 8
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, ptr %x1)
  ret i32 0
}
