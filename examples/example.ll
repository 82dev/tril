; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [5 x i8] c"aaaa\00", align 1
@str.1 = private unnamed_addr constant [5 x i8] c"aaaa\00", align 1

declare float @puts(ptr)

declare float @printf(ptr)

define float @main() {
entry:
  %a = alloca float, align 4
  store float 2.000000e+00, ptr %a, align 4
  %printf = call float @printf(ptr @str)
  %printf1 = call float @printf(ptr @str.1)
  ret float 0.000000e+00
}
