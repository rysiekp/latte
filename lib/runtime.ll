; ModuleID = 'runtime.c'

%struct.__sFILE = type { i8*, i32, i32, i16, i16, %struct.__sbuf, i32, i8*, i32 (i8*)*, i32 (i8*, i8*, i32)*, i64 (i8*, i64, i32)*, i32 (i8*, i8*, i32)*, %struct.__sbuf, %struct.__sFILEX*, i32, [3 x i8], [1 x i8], %struct.__sbuf, i32, i64 }
%struct.__sFILEX = type opaque
%struct.__sbuf = type { i8*, i32 }

@.str = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [3 x i8] c"%d\00", align 1
@__stdinp = external global %struct.__sFILE*, align 8
@.str.3 = private unnamed_addr constant [15 x i8] c"runtime error\0A\00", align 1

; Function Attrs: nounwind ssp uwtable
define i8* @.concat(i8*, i8*) #0 {
  %3 = alloca i8*, align 8
  %4 = alloca i8*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i8*, align 8
  store i8* %0, i8** %4, align 8
  store i8* %1, i8** %5, align 8
  %9 = load i8*, i8** %4, align 8
  %10 = call i64 @strlen(i8* %9)
  store i64 %10, i64* %6, align 8
  %11 = load i8*, i8** %5, align 8
  %12 = call i64 @strlen(i8* %11)
  store i64 %12, i64* %7, align 8
  %13 = load i64, i64* %6, align 8
  %14 = load i64, i64* %7, align 8
  %15 = add i64 %13, %14
  %16 = add i64 %15, 1
  %17 = call i8* @malloc(i64 %16)
  store i8* %17, i8** %8, align 8
  %18 = load i8*, i8** %8, align 8
  %19 = icmp eq i8* %18, null
  br i1 %19, label %20, label %21

; <label>:20                                      ; preds = %2
  store i8* null, i8** %3, align 8
  br label %43

; <label>:21                                      ; preds = %2
  %22 = load i8*, i8** %8, align 8
  %23 = load i8*, i8** %4, align 8
  %24 = load i8*, i8** %8, align 8
  %25 = call i64 @llvm.objectsize.i64.p0i8(i8* %24, i1 false)
  %26 = call i8* @__strcpy_chk(i8* %22, i8* %23, i64 %25) #5
  %27 = load i8*, i8** %8, align 8
  %28 = load i64, i64* %6, align 8
  %29 = getelementptr inbounds i8, i8* %27, i64 %28
  %30 = load i8*, i8** %5, align 8
  %31 = load i8*, i8** %8, align 8
  %32 = load i64, i64* %6, align 8
  %33 = getelementptr inbounds i8, i8* %31, i64 %32
  %34 = call i64 @llvm.objectsize.i64.p0i8(i8* %33, i1 false)
  %35 = call i8* @__strcpy_chk(i8* %29, i8* %30, i64 %34) #5
  %36 = load i64, i64* %6, align 8
  %37 = load i64, i64* %7, align 8
  %38 = add i64 %36, %37
  %39 = add i64 %38, 1
  %40 = load i8*, i8** %8, align 8
  %41 = getelementptr inbounds i8, i8* %40, i64 %39
  store i8 0, i8* %41, align 1
  %42 = load i8*, i8** %8, align 8
  store i8* %42, i8** %3, align 8
  br label %43

; <label>:43                                      ; preds = %21, %20
  %44 = load i8*, i8** %3, align 8
  ret i8* %44
}

declare i64 @strlen(i8*) #1

declare i8* @malloc(i64) #1

; Function Attrs: nounwind
declare i8* @__strcpy_chk(i8*, i8*, i64) #2

; Function Attrs: nounwind readnone
declare i64 @llvm.objectsize.i64.p0i8(i8*, i1) #3

; Function Attrs: nounwind ssp uwtable
define void @printString(i8*) #0 {
  %2 = alloca i8*, align 8
  store i8* %0, i8** %2, align 8
  %3 = load i8*, i8** %2, align 8
  %4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i8* %3)
  ret void
}

declare i32 @printf(i8*, ...) #1

; Function Attrs: nounwind ssp uwtable
define void @printInt(i32) #0 {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = load i32, i32* %2, align 4
  %4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i32 %3)
  ret void
}

; Function Attrs: nounwind ssp uwtable
define i32 @readInt() #0 {
  %1 = alloca i32, align 4
  %2 = call i32 (i8*, ...) @scanf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.str.2, i32 0, i32 0), i32* %1)
  %3 = call i32 @getchar()
  %4 = load i32, i32* %1, align 4
  ret i32 %4
}

declare i32 @scanf(i8*, ...) #1

declare i32 @getchar() #1

; Function Attrs: nounwind ssp uwtable
define i8* @readString() #0 {
  %1 = alloca i8*, align 8
  %2 = alloca i8*, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i8* null, i8** %2, align 8
  store i64 0, i64* %3, align 8
  %5 = load %struct.__sFILE*, %struct.__sFILE** @__stdinp, align 8
  %6 = call i64 @getline(i8** %2, i64* %3, %struct.__sFILE* %5)
  %7 = icmp eq i64 %6, -1
  br i1 %7, label %8, label %9

; <label>:8                                       ; preds = %0
  store i8* null, i8** %1, align 8
  br label %24

; <label>:9                                       ; preds = %0
  %10 = load i8*, i8** %2, align 8
  %11 = call i64 @strlen(i8* %10)
  store i64 %11, i64* %4, align 8
  %12 = load i8*, i8** %2, align 8
  %13 = getelementptr inbounds i8, i8* %12, i64 0
  %14 = load i8, i8* %13, align 1
  %15 = sext i8 %14 to i32
  %16 = icmp ne i32 %15, 10
  br i1 %16, label %17, label %22

; <label>:17                                      ; preds = %9
  %18 = load i64, i64* %4, align 8
  %19 = sub i64 %18, 1
  %20 = load i8*, i8** %2, align 8
  %21 = getelementptr inbounds i8, i8* %20, i64 %19
  store i8 0, i8* %21, align 1
  br label %22

; <label>:22                                      ; preds = %17, %9
  %23 = load i8*, i8** %2, align 8
  store i8* %23, i8** %1, align 8
  br label %24

; <label>:24                                      ; preds = %22, %8
  %25 = load i8*, i8** %1, align 8
  ret i8* %25
}

declare i64 @getline(i8**, i64*, %struct.__sFILE*) #1

; Function Attrs: nounwind ssp uwtable
define void @error() #0 {
  %1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.str.3, i32 0, i32 0))
  call void @exit(i32 -1) #6
  unreachable
                                                  ; No predecessors!
  ret void
}

; Function Attrs: noreturn
declare void @exit(i32) #4

attributes #0 = { nounwind ssp uwtable "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+mmx,+sse,+sse2,+sse3,+sse4.1,+ssse3" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+mmx,+sse,+sse2,+sse3,+sse4.1,+ssse3" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { nounwind "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+mmx,+sse,+sse2,+sse3,+sse4.1,+ssse3" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind readnone }
attributes #4 = { noreturn "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+mmx,+sse,+sse2,+sse3,+sse4.1,+ssse3" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #5 = { nounwind }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"PIC Level", i32 2}
!1 = !{!"Apple LLVM version 8.0.0 (clang-800.0.42.1)"}
