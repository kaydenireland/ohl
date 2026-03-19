; ModuleID = 'ohl'
source_filename = "ohl"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 0, ptr %x, align 4
  br label %do_body

do_body:                                          ; preds = %do_cond, %entry
  %x1 = load i32, ptr %x, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, i32 %x1)
  %x2 = load i32, ptr %x, align 4
  %add = add i32 %x2, 1
  store i32 %add, ptr %x, align 4
  br label %do_cond

do_cond:                                          ; preds = %do_body
  %x3 = load i32, ptr %x, align 4
  %gt = icmp sgt i32 %x3, 5
  br i1 %gt, label %do_body, label %do_end

do_end:                                           ; preds = %do_cond
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
