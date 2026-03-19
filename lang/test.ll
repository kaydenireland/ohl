; ModuleID = 'ohl'
source_filename = "ohl"

@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@fmt.1 = private unnamed_addr constant [4 x i8] c"%c\0A\00", align 1
@true_str = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %c = alloca i16, align 2
  %s = alloca ptr, align 8
  store ptr null, ptr %s, align 8
  %s1 = load ptr, ptr %s, align 8
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, ptr %s1)
  store ptr null, ptr %c, align 8
  %c2 = load i16, ptr %c, align 2
  %char_ext = sext i16 %c2 to i32
  %printf_call3 = call i32 (ptr, ...) @printf(ptr @fmt.1, i32 %char_ext)
  call void @foo()
  ret i32 0
}

define void @foo() {
entry:
  %even = alloca i1, align 1
  %x = alloca i32, align 4
  store i32 4, ptr %x, align 4
  %x1 = load i32, ptr %x, align 4
  %calltmp = call i1 @isEven(i32 %x1)
  store i1 %calltmp, ptr %even, align 1
  %even2 = load i1, ptr %even, align 1
  br i1 %even2, label %bool_true, label %bool_false

bool_true:                                        ; preds = %entry
  %print_true = call i32 (ptr, ...) @printf(ptr @true_str)
  br label %bool_merge

bool_false:                                       ; preds = %entry
  %print_false = call i32 (ptr, ...) @printf(ptr @false_str)
  br label %bool_merge

bool_merge:                                       ; preds = %bool_false, %bool_true
  ret void
}

define i1 @isEven(i32 %x) {
entry:
  %rem = alloca i32, align 4
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %x2 = load i32, ptr %x1, align 4
  %mod = srem i32 %x2, 2
  store i32 %mod, ptr %rem, align 4
  %rem3 = load i32, ptr %rem, align 4
  %eq = icmp eq i32 %rem3, 0
  br i1 %eq, label %then, label %merge

then:                                             ; preds = %entry
  ret i1 true

merge:                                            ; preds = %entry
  ret i1 false
}
