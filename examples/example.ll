; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [4 x i8] c"%c \00", align 1

declare void @puts(ptr)

declare void @printf(ptr, i32)

define void @print_num(i32 %n) {
entry:
  %n1 = alloca i32, align 4
  store i32 %n, ptr %n1, align 4
  %n2 = load i32, ptr %n1, align 4
  call void @printf(ptr @str, i32 %n2)
  ret void
}

define void @main() {
entry:
  %i = alloca i32, align 4
  store i32 90, ptr %i, align 4
  br label %whilecond

whilecond:                                        ; preds = %whileloop, %entry
  %i1 = load i32, ptr %i, align 4
  %igt = icmp sgt i32 %i1, 64
  %whilecond2 = icmp ne i1 %igt, false
  br i1 %whilecond2, label %whileloop, label %afterwhile

whileloop:                                        ; preds = %whilecond
  %i3 = load i32, ptr %i, align 4
  call void @print_num(i32 %i3)
  %i4 = load i32, ptr %i, align 4
  %isub = sub i32 %i4, 1
  store i32 %isub, ptr %i, align 4
  br label %whilecond

afterwhile:                                       ; preds = %whilecond
  ret void
}
