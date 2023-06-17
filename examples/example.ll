; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [4 x i8] c"%d \00", align 1

declare void @printf(ptr, i32)

define i32 @main() {
entry:
  %x = alloca [3 x i32], align 4
  %arrliteral = alloca [3 x i32], align 4
  %gep = getelementptr i32, ptr %arrliteral, i32 0
  store i32 -31415, ptr %gep, align 4
  %gep1 = getelementptr i32, ptr %arrliteral, i32 1
  store i32 42, ptr %gep1, align 4
  %gep2 = getelementptr i32, ptr %arrliteral, i32 2
  store i32 73, ptr %gep2, align 4
  %load = load [3 x i32], ptr %arrliteral, align 4
  store [3 x i32] %load, ptr %x, align 4
  %gep_index = getelementptr i32, ptr %x, i32 0
  %index = load i32, ptr %gep_index, align 4
  call void @printf(ptr @str, i32 %index)
  ret i32 0
}
