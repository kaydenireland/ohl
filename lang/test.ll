; ModuleID = 'ohl'
source_filename = "ohl"

@true_str = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1
@true_str.1 = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str.2 = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1
@true_str.3 = private unnamed_addr constant [6 x i8] c"true\0A\00", align 1
@false_str.4 = private unnamed_addr constant [7 x i8] c"false\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %x = alloca i1, align 1
  %z = alloca i1, align 1
  store i1 true, ptr %z, align 1
  store i1 false, ptr %x, align 1
  %z1 = load i1, ptr %z, align 1
  br i1 %z1, label %bool_true, label %bool_false

bool_true:                                        ; preds = %entry
  %print_true = call i32 (ptr, ...) @printf(ptr @true_str)
  br label %bool_merge

bool_false:                                       ; preds = %entry
  %print_false = call i32 (ptr, ...) @printf(ptr @false_str)
  br label %bool_merge

bool_merge:                                       ; preds = %bool_false, %bool_true
  %x2 = load i1, ptr %x, align 1
  br i1 %x2, label %bool_true3, label %bool_false4

bool_true3:                                       ; preds = %bool_merge
  %print_true6 = call i32 (ptr, ...) @printf(ptr @true_str.1)
  br label %bool_merge5

bool_false4:                                      ; preds = %bool_merge
  %print_false7 = call i32 (ptr, ...) @printf(ptr @false_str.2)
  br label %bool_merge5

bool_merge5:                                      ; preds = %bool_false4, %bool_true3
  br i1 true, label %bool_true8, label %bool_false9

bool_true8:                                       ; preds = %bool_merge5
  %print_true11 = call i32 (ptr, ...) @printf(ptr @true_str.3)
  br label %bool_merge10

bool_false9:                                      ; preds = %bool_merge5
  %print_false12 = call i32 (ptr, ...) @printf(ptr @false_str.4)
  br label %bool_merge10

bool_merge10:                                     ; preds = %bool_false9, %bool_true8
  ret i32 0
}
