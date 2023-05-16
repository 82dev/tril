; ModuleID = 'example.tril'
source_filename = "example.tril"

define float @foo(float %a) {
entry:
  %a1 = alloca float, align 4
  store float %a, ptr %a1, align 4
  %b = alloca float, align 4
  store float 3.000000e+00, ptr %b, align 4
  %b2 = load float, ptr %b, align 4
  ret float %b2
}

define float @main(float %a) {
entry:
  %a1 = alloca float, align 4
  store float %a, ptr %a1, align 4
  %v = alloca float, align 4
  %foo = call float @foo(float 2.000000e+00)
  store float %foo, ptr %v, align 4
  %a2 = load float, ptr %a1, align 4
  ret float %a2
}
