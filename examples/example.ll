; ModuleID = 'example'
source_filename = "example"

define float @foo(float %a) {
entry:
  %a1 = alloca float, align 4
  store float %a, ptr %a1, align 4
  %a2 = load float, ptr %a1, align 4
  %addtmp = fadd float %a2, 2.000000e+00
  ret float %addtmp
}

define float @main(float %a) {
entry:
  %a1 = alloca float, align 4
  store float %a, ptr %a1, align 4
  %b = alloca float, align 4
  %a2 = load float, ptr %a1, align 4
  %foo = call float @foo(float %a2)
  store float %foo, ptr %b, align 4
  %b3 = load float, ptr %b, align 4
  ret float %b3
}
