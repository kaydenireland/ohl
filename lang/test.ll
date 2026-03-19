; ModuleID = 'ohl'
source_filename = "ohl"

@true_str = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1
@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@true_str.1 = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str.2 = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 99, ptr %x, align 4
  %x1 = load i32, ptr %x, align 4
  %eq = icmp eq i32 %x1, 5
  br i1 %eq, label %then, label %else

then:                                             ; preds = %entry
  br i1 true, label %bool_true, label %bool_false

merge:                                            ; preds = %merge5, %bool_merge
  ret i32 0

else:                                             ; preds = %entry
  %x2 = load i32, ptr %x, align 4
  %eq3 = icmp eq i32 %x2, 3
  br i1 %eq3, label %then4, label %else6

bool_true:                                        ; preds = %then
  %print_true = call i32 (ptr, ...) @printf(ptr @true_str)
  br label %bool_merge

bool_false:                                       ; preds = %then
  %print_false = call i32 (ptr, ...) @printf(ptr @false_str)
  br label %bool_merge

bool_merge:                                       ; preds = %bool_false, %bool_true
  br label %merge

then4:                                            ; preds = %else
  %x7 = load i32, ptr %x, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, i32 %x7)
  br label %merge5

merge5:                                           ; preds = %bool_merge10, %then4
  br label %merge

else6:                                            ; preds = %else
  br i1 false, label %bool_true8, label %bool_false9

bool_true8:                                       ; preds = %else6
  %print_true11 = call i32 (ptr, ...) @printf(ptr @true_str.1)
  br label %bool_merge10

bool_false9:                                      ; preds = %else6
  %print_false12 = call i32 (ptr, ...) @printf(ptr @false_str.2)
  br label %bool_merge10

bool_merge10:                                     ; preds = %bool_false9, %bool_true8
  br label %merge5
}
