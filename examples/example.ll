; ModuleID = 'example'
source_filename = "example"

declare float @print_ascii(float)

define float @main(float %a) {
entry:
  %a1 = alloca float, align 4
  store float %a, ptr %a1, align 4
  %a2 = alloca float, align 4
  store float 6.200000e+01, ptr %a2, align 4
  %a3 = alloca float, align 4
  store float 7.800000e+01, ptr %a3, align 4
  %x = alloca float, align 4
  %a4 = load float, ptr %a3, align 4
  %print_ascii = call float @print_ascii(float %a4)
  store float %print_ascii, ptr %x, align 4
  ret float 0.000000e+00
}
