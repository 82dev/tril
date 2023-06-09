; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@str.1 = private unnamed_addr constant [3 x i8] c"%d\00", align 1

declare void @printf(ptr, i32)

define i32 @inc(i32 %x) {
entry:
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %x2 = load i32, ptr %x1, align 4
  %iadd = add i32 %x2, 1
  ret i32 %iadd
}

define void @main() {
entry:
  %x = alloca i32, align 4
  %inc = call i32 @inc(i32 2)
  store i32 %inc, ptr %x, align 4
  %y = alloca i32, align 4
  %x1 = load i32, ptr %x, align 4
  %inc2 = call i32 @inc(i32 %x1)
  store i32 %inc2, ptr %y, align 4
  %x3 = load i32, ptr %x, align 4
  call void @printf(ptr @str, i32 %x3)
  %y4 = load i32, ptr %y, align 4
  call void @printf(ptr @str.1, i32 %y4)
  ret void
}
