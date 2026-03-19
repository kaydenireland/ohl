; ModuleID = 'ohl'
source_filename = "ohl"

@true_str = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %even = alloca i1, align 1
  %x = alloca i32, align 4
  store i32 4, ptr %x, align 4
  %calltmp = call i1 @isEven(i1 true)
  %calltmp1 = call i1 @isEven(i1 true)
  store i1 %calltmp1, ptr %even, align 1
  %even2 = load i1, ptr %even, align 1
  br i1 %even2, label %bool_true, label %bool_false

bool_true:                                        ; preds = %entry
  %print_true = call i32 (ptr, ...) @printf(ptr @true_str)
  br label %bool_merge

bool_false:                                       ; preds = %entry
  %print_false = call i32 (ptr, ...) @printf(ptr @false_str)
  br label %bool_merge

bool_merge:                                       ; preds = %bool_false, %bool_true
  ret i32 0
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
