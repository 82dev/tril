; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [5 x i8] c"%d \0A\00", align 1

declare void @printf(ptr, i32)

define i32 @foo([3 x i32] %x) {
entry:
  %x1 = alloca [3 x i32], align 4
  store [3 x i32] %x, ptr %x1, align 4
  %gep_index = getelementptr i32, ptr %x1, i32 0
  %index = load i32, ptr %gep_index, align 4
  ret i32 %index
}

define i32 @main() {
entry:
  %x = alloca [3 x i32], align 4
  %arrliteral = alloca [3 x i32], align 4
  %gep = getelementptr i32, ptr %arrliteral, i32 0
  store i32 69, ptr %gep, align 4
  %gep1 = getelementptr i32, ptr %arrliteral, i32 1
  store i32 3, ptr %gep1, align 4
  %gep2 = getelementptr i32, ptr %arrliteral, i32 2
  store i32 5, ptr %gep2, align 4
  %load = load [3 x i32], ptr %arrliteral, align 4
  store [3 x i32] %load, ptr %x, align 4
  %x3 = load [3 x i32], ptr %x, align 4
  %foo = call i32 @foo([3 x i32] %x3)
  call void @printf(ptr @str, i32 %foo)
  ret i32 0
}
