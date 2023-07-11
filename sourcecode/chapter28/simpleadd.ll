; ModuleID = 'simpleadd.c50fed6e-cgu.0'
source_filename = "simpleadd.c50fed6e-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha4931af2a5a0138fE", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h66f0828d1091c7e0E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc323db7abb16ad90E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc323db7abb16ad90E" }>, align 8
@alloc_09d11aa498739cbf0519d318f9792c9b = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_71b99a1804d93c013f301ec59bc480be = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_09d11aa498739cbf0519d318f9792c9b, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc_2bfeba76c1438a46208a034153050220 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc_d712c9c13286bcada7944d76d166911e = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc\\library\\core\\src\\fmt\\mod.rs" }>, align 1
@alloc_c2b79287c08f2705fa5817e5edf6f560 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_d712c9c13286bcada7944d76d166911e, [16 x i8] c"K\00\00\00\00\00\00\00\93\01\00\00\0D\00\00\00" }>, align 8
@alloc_fdd68f548079bc2689cae613ac9cb40d = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c".\\simpleadd.rs" }>, align 1
@alloc_d96ec9b5112fdcc55cb9d6d4b7f83e42 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_fdd68f548079bc2689cae613ac9cb40d, [16 x i8] c"\0E\00\00\00\00\00\00\00\02\00\00\00\0A\00\00\00" }>, align 8
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc_a258a5fed9db0b8414d744b8384f7e10 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_0f090c17f54ee31bf6c742813e9803d6 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_2bfeba76c1438a46208a034153050220, [8 x i8] zeroinitializer, ptr @alloc_a258a5fed9db0b8414d744b8384f7e10, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha051e4f255f98a37E(ptr %f) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h35af58ab670c183aE(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !2
  br label %bb4

bb4:                                              ; preds = %start
  ret void

bb2:                                              ; preds = %funclet_bb2
  cleanupret from %cleanuppad unwind to caller

funclet_bb2:                                      ; No predecessors!
  %cleanuppad = cleanuppad within none []
  br label %bb2
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17h654927aeeb6ee037E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
start:
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h1149c819db815d35E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8, !noundef !3
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc323db7abb16ad90E"(ptr align 8 %_1) unnamed_addr #2 {
start:
  %self = alloca i32, align 4
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha051e4f255f98a37E(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hd7764199c4210f65E"()
  store i32 %0, ptr %self, align 4
  %_6 = load i32, ptr %self, align 4, !noundef !3
  ret i32 %_6
}

; core::fmt::ArgumentV1::new_display
; Function Attrs: inlinehint uwtable
define internal { ptr, ptr } @_ZN4core3fmt10ArgumentV111new_display17h97a02ad9a30d6baaE(ptr align 4 %x) unnamed_addr #2 {
start:
  %0 = alloca ptr, align 8
  %1 = alloca ptr, align 8
  %2 = alloca { ptr, ptr }, align 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h0dbe31796eb50397E", ptr %1, align 8
  %_3 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
  store ptr %x, ptr %0, align 8
  %_4 = load ptr, ptr %0, align 8, !nonnull !3, !align !4, !noundef !3
  store ptr %_4, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  store ptr %_3, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 0
  %5 = load ptr, ptr %4, align 8, !nonnull !3, !align !4, !noundef !3
  %6 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  %7 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
  %8 = insertvalue { ptr, ptr } undef, ptr %5, 0
  %9 = insertvalue { ptr, ptr } %8, ptr %7, 1
  ret { ptr, ptr } %9
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hf648f379a6b39a00E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #2 {
start:
  %_15 = alloca { ptr, i64 }, align 8
  %_12 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_9 = add i64 %args.1, 1
  %_7 = icmp ugt i64 %pieces.1, %_9
  %1 = zext i1 %_7 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !5, !noundef !3
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  store ptr null, ptr %_15, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !6, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hf648f379a6b39a00E(ptr sret(%"core::fmt::Arguments<'_>") %_12, ptr align 8 @alloc_71b99a1804d93c013f301ec59bc480be, i64 1, ptr align 8 @alloc_2bfeba76c1438a46208a034153050220, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h800ffd4974e06f2dE(ptr %_12, ptr align 8 @alloc_c2b79287c08f2705fa5817e5edf6f560) #7
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h66f0828d1091c7e0E"(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h18a24e66ab1e3910E(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h18a24e66ab1e3910E(ptr %0) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %1 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc323db7abb16ad90E"(ptr align 8 %_1)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  cleanupret from %cleanuppad unwind to caller

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h35af58ab670c183aE(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha4931af2a5a0138fE"(ptr %_1) unnamed_addr #2 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hd7764199c4210f65E"() unnamed_addr #2 {
start:
  ret i32 0
}

; simpleadd::simple_add
; Function Attrs: uwtable
define internal i32 @_ZN9simpleadd10simple_add17hd5e3cd3c04366853E(i32 %x, i32 %y) unnamed_addr #1 {
start:
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %x, i32 %y)
  %_3.0 = extractvalue { i32, i1 } %0, 0
  %_3.1 = extractvalue { i32, i1 } %0, 1
  %1 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false)
  br i1 %1, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_3.0

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h3445eebb4065373cE(ptr align 1 @str.1, i64 28, ptr align 8 @alloc_d96ec9b5112fdcc55cb9d6d4b7f83e42) #7
  unreachable
}

; simpleadd::main
; Function Attrs: uwtable
define internal void @_ZN9simpleadd4main17hd5819ed649506ac1E() unnamed_addr #1 {
start:
  %_7 = alloca [1 x { ptr, ptr }], align 8
  %_3 = alloca %"core::fmt::Arguments<'_>", align 8
  %z = alloca i32, align 4
; call simpleadd::simple_add
  %0 = call i32 @_ZN9simpleadd10simple_add17hd5e3cd3c04366853E(i32 3, i32 4)
  store i32 %0, ptr %z, align 4
; call core::fmt::ArgumentV1::new_display
  %1 = call { ptr, ptr } @_ZN4core3fmt10ArgumentV111new_display17h97a02ad9a30d6baaE(ptr align 4 %z)
  %_8.0 = extractvalue { ptr, ptr } %1, 0
  %_8.1 = extractvalue { ptr, ptr } %1, 1
  %2 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_7, i64 0, i64 0
  %3 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 0
  store ptr %_8.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  store ptr %_8.1, ptr %4, align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hf648f379a6b39a00E(ptr sret(%"core::fmt::Arguments<'_>") %_3, ptr align 8 @alloc_0f090c17f54ee31bf6c742813e9803d6, i64 2, ptr align 8 %_7, i64 1)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hce7a376ab49946d5E(ptr %_3)
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #3

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h1149c819db815d35E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h0dbe31796eb50397E"(ptr align 4, ptr align 8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17h800ffd4974e06f2dE(ptr, ptr align 8) unnamed_addr #4

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #5

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #6

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h3445eebb4065373cE(ptr align 1, i64, ptr align 8) unnamed_addr #4

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17hce7a376ab49946d5E(ptr) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #3 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h654927aeeb6ee037E(ptr @_ZN9simpleadd4main17hd5819ed649506ac1E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #3 = { "target-cpu"="x86-64" }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #5 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #6 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 1666892}
!3 = !{}
!4 = !{i64 1}
!5 = !{i8 0, i8 2}
!6 = !{i64 8}
