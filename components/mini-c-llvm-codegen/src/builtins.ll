declare i32 @putchar(i8)

define void @println() {
entry:
  ; call void @print(i64 %0)
  call i32 @putchar(i8 10)
  ret void
}

define void @print(i64 %0) {
entry:
  %corner_case = icmp eq i64 %0, 0
  br i1 %corner_case, label %corner_case_number_is_zero, label %usual_flow

usual_flow:                                       ; preds = %entry
  %p_input = alloca i64, align 8
  store i64 %0, ptr %p_input, align 4
  br label %block_while_condition_1

block_while_condition_1:                          ; preds = %block_while_scope_2, %usual_flow
  %1 = load i64, ptr %p_input, align 4
  %2 = icmp ne i64 %1, 0
  br i1 %2, label %block_while_scope_2, label %print_end

block_while_scope_2:                              ; preds = %block_while_condition_1
  %remainder = srem i64 %1, 10
  %remainder_char = trunc i64 %remainder to i8
  %remainder_i8_ascii = add i8 %remainder_char, 48
  call i32 @putchar(i8 %remainder_i8_ascii)
  %new_input_value = sdiv i64 %1, 10
  store i64 %new_input_value, ptr %p_input, align 4
  br label %block_while_condition_1

corner_case_number_is_zero:                       ; preds = %entry
  call i32 @putchar(i8 48) ; 48 = b'0'
  br label %print_end

print_end:                                ; preds = %block_while_condition_1, corner_case_number_is_zero
  ret void
}

