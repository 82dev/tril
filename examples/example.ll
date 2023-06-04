; ModuleID = 'example'
source_filename = "example"

@str = private unnamed_addr constant [12 x i8] c"Hello bitch\00", align 1
@str.1 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1

declare float @puts(ptr)

define float @main() {
entry:
  %a = alloca float, align 4
  store float 3.000000e+00, ptr %a, align 4
  %b = alloca float, align 4
  store float 2.000000e+00, ptr %b, align 4
  br i1 true, label %then, label %else

then:                                             ; preds = %entry
  %puts = call float @puts(ptr @str)
  br label %merge

else:                                             ; preds = %entry
  %puts1 = call float @puts(ptr @str.1)
  br label %merge

merge:                                            ; preds = %else, %then
  ret float 0.000000e+00
}
