; ModuleID = 'example'
source_filename = "example"

define { i32, i32 } @vec2(i32 %x, i32 %y) {
entry:
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %y2 = alloca i32, align 4
  store i32 %y, ptr %y2, align 4
  %Vec2 = alloca { i32, i32 }, align 8
  %x3 = load i32, ptr %x1, align 4
  %structgep = getelementptr inbounds { i32, i32 }, ptr %Vec2, i32 0, i32 0
  store i32 %x3, ptr %structgep, align 4
  %y4 = load i32, ptr %y2, align 4
  %structgep5 = getelementptr inbounds { i32, i32 }, ptr %Vec2, i32 0, i32 1
  store i32 %y4, ptr %structgep5, align 4
  ret ptr %Vec2
}
