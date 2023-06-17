; ModuleID = './opt.ll'
source_filename = "example"

define i32 @main() {
entry:
  %x = alloca [3 x i32], align 4
  %arr_alloca = alloca [3 x i32], align 4
  %arr = load [3 x i32], ptr %arr_alloca, align 4
  %"elem_{i}" = insertvalue [3 x i32] %arr, i32 2, 0
  %"elem_{i}1" = insertvalue [3 x i32] %arr, i32 3, 1
  %"elem_{i}2" = insertvalue [3 x i32] %arr, i32 4, 2
  store [3 x i32] %arr, ptr %x, align 4
  %y = alloca [3 x i32], align 4
  %x3 = load [3 x i32], ptr %x, align 4
  store [3 x i32] %x3, ptr %y, align 4
  ret i32 0
}
