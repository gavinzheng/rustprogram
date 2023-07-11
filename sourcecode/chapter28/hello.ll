@msg = constant [12 x i8] c"hello world\00"

declare i32 @puts(i8*)

define i32 @main() {
    %str = getelementptr [12 x i8],[12 x i8]* @msg, i64 0, i64 0

    call i32 @puts(i8* %str)
    ret i32 0
}