# Rust Assignment

## Lesson 1

- Assignment 1: source code
- Assignment 2: source code

**Bài tập 1:** Cho 2 mảng có các phần tử là số nguyên dương, kiểm tra mảng này có phải là mảng con của mảng kia không ?(yêu cầu đúng thứ tự của các phần tử)
- let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
- let sub_arr = [6 , 8 , 10];
  - Duyệt từng mảng con trong org_arr: i=j=0
    - org[i] = sub[j]
      - i++
      - j++
      - j==len(sub) -> true
    - org[i] !=sub[j]
      - i=i-j+1
      - Xét lại j=0

**Bài tập 2:** Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
Ví dụ: let input = “adbcdaDd”. 
- Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”
- Nhập s = ‘d’ => in ra kết quả : 4, “abca”

- Remove s (uppper, lower) trong chuỗi
- count: độ dài thay đổi

## Lession 2

Assigement 3,4,5,6: Đã sửa lỗi